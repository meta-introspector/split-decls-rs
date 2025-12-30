// Generated macro for spawn (function)
macro_rules! Depcratespawn {
() => {
// Module: crate
// Provides: {"spawn"}
// Dependencies: {}
# [doc = " Spawns a future on the current thread as a new task."] # [doc = ""] # [doc = " If not immediately awaited, the task can be cancelled using [`JoinHandle::abort`]."] # [doc = ""] # [doc = " The provided future is spawned as a new task; therefore, panics are caught."] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if Actix system is not running."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " # use std::time::Duration;"] # [doc = " # actix_rt::Runtime::new().unwrap().block_on(async {"] # [doc = " // task resolves successfully"] # [doc = " assert_eq!(actix_rt::spawn(async { 1 }).await.unwrap(), 1);"] # [doc = ""] # [doc = " // task panics"] # [doc = " assert!(actix_rt::spawn(async {"] # [doc = "     panic!(\"panic is caught at task boundary\");"] # [doc = " })"] # [doc = " .await"] # [doc = " .unwrap_err()"] # [doc = " .is_panic());"] # [doc = ""] # [doc = " // task is cancelled before completion"] # [doc = " let handle = actix_rt::spawn(actix_rt::time::sleep(Duration::from_secs(100)));"] # [doc = " handle.abort();"] # [doc = " assert!(handle.await.unwrap_err().is_cancelled());"] # [doc = " # });"] # [doc = " ```"] # [track_caller] # [inline] pub fn spawn < Fut > (f : Fut) -> JoinHandle < Fut :: Output > where Fut : Future + 'static , Fut :: Output : 'static , { tokio :: task :: spawn_local (f) }
};
}
