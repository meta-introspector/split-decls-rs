// Generated macro for noop_spawner_mut (function)
macro_rules! Depcrate_task_noop_spawnernoop_spawner_mut {
() => {
// Module: crate::task::noop_spawner
// Provides: {"noop_spawner_mut"}
// Dependencies: {}
# [doc = " Get a reference to a singleton instance of [`NoopSpawner`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures::task::SpawnExt;"] # [doc = " use futures_test::task::noop_spawner_mut;"] # [doc = ""] # [doc = " let spawner = noop_spawner_mut();"] # [doc = " spawner.spawn(async { }).unwrap();"] # [doc = " ```"] pub fn noop_spawner_mut () -> & 'static mut NoopSpawner { Box :: leak (Box :: new (NoopSpawner :: new ())) }
};
}
