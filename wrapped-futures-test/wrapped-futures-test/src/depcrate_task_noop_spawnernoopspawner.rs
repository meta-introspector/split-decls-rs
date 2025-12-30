// Generated macro for NoopSpawner (struct)
macro_rules! Depcrate_task_noop_spawnerNoopSpawner {
() => {
// Module: crate::task::noop_spawner
// Provides: {"NoopSpawner"}
// Dependencies: {}
# [doc = " An implementation of [`Spawn`](futures_task::Spawn) that"] # [doc = " discards spawned futures when used."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures::task::SpawnExt;"] # [doc = " use futures_test::task::NoopSpawner;"] # [doc = ""] # [doc = " let spawner = NoopSpawner::new();"] # [doc = " spawner.spawn(async { }).unwrap();"] # [doc = " ```"] # [derive (Debug)] pub struct NoopSpawner { _reserved : () , }
};
}
