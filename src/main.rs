use avian2d::prelude::*;
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            // Enables debug rendering
            PhysicsDebugPlugin,
        ))
        .insert_resource(Gravity(Vec2::ZERO))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .run()
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Projectile;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Block;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // setup camera
    commands.spawn(Camera2d);

    let material = materials.add(Color::WHITE);

    // setup bocks
    let block_shape = Rectangle::from_length(50.);
    let block_mesh = meshes.add(block_shape.clone());

    commands
        .spawn((
            Block,
            Transform::from_xyz(200., 0., 0.),
            RigidBody::Static,
            Collider::from(block_shape),
            Mesh2d(block_mesh.clone()),
            MeshMaterial2d(material.clone()),
            CollisionEventsEnabled,
        ))
        .observe(despawn_on_collide);

    commands
        .spawn((
            Block,
            Transform::from_xyz(300., 0., 0.),
            RigidBody::Static,
            Collider::from(block_shape),
            Mesh2d(block_mesh),
            MeshMaterial2d(material.clone()),
            CollisionEventsEnabled,
        ))
        .observe(despawn_on_collide);

    // setup and accelerate projectiles
    let projectile_shape = Circle::new(10.);
    let projectile_mesh = meshes.add(projectile_shape.clone());

    commands.spawn((
        Projectile,
        RigidBody::Dynamic,
        Collider::from(projectile_shape),
        LinearVelocity(Vec2::X * 50.),
        Mesh2d(projectile_mesh.clone()),
        MeshMaterial2d(material.clone()),
    ));

    commands.spawn((
        Projectile,
        Transform::from_xyz(-100., 0., 0.),
        RigidBody::Dynamic,
        Collider::from(projectile_shape),
        LinearVelocity(Vec2::X * 50.),
        Mesh2d(projectile_mesh),
        MeshMaterial2d(material),
    ));
}

fn despawn_on_collide(event: On<CollisionStart>, mut commands: Commands) {
    commands.entity(event.collider1).despawn();
    commands.entity(event.collider2).despawn();
}
