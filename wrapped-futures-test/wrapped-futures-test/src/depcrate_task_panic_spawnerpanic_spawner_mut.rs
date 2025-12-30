// Generated macro for panic_spawner_mut (function)
macro_rules! Depcrate_task_panic_spawnerpanic_spawner_mut {
() => {
// Module: crate::task::panic_spawner
// Provides: {"panic_spawner_mut"}
// Dependencies: {}
# [doc = " Get a reference to a singleton instance of [`PanicSpawner`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use futures::task::SpawnExt;"] # [doc = " use futures_test::task::panic_spawner_mut;"] # [doc = ""] # [doc = " let spawner = panic_spawner_mut();"] # [doc = " spawner.spawn(async { })?; // Will panic"] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub fn panic_spawner_mut () -> & 'static mut PanicSpawner { Box :: leak (Box :: new (PanicSpawner :: new ())) }
};
}
