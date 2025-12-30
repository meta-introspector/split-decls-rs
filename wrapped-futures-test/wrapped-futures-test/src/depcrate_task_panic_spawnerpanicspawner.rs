// Generated macro for PanicSpawner (struct)
macro_rules! Depcrate_task_panic_spawnerPanicSpawner {
() => {
// Module: crate::task::panic_spawner
// Provides: {"PanicSpawner"}
// Dependencies: {}
# [doc = " An implementation of [`Spawn`](futures_task::Spawn) that panics"] # [doc = " when used."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use futures::task::SpawnExt;"] # [doc = " use futures_test::task::PanicSpawner;"] # [doc = ""] # [doc = " let spawn = PanicSpawner::new();"] # [doc = " spawn.spawn(async { })?; // Will panic"] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [derive (Debug)] pub struct PanicSpawner { _reserved : () , }
};
}
