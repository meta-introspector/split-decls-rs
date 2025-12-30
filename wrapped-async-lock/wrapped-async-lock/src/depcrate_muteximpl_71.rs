// Generated macro for impl_71 (impl)
macro_rules! Depcrate_muteximpl_71 {
() => {
// Module: crate::mutex
// Provides: {"impl_71"}
// Dependencies: {}
impl < T : ? Sized > MutexGuardArc < T > { # [doc = " Returns a reference to the mutex a guard came from."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use async_lock::{Mutex, MutexGuardArc};"] # [doc = " use std::sync::Arc;"] # [doc = ""] # [doc = " let mutex = Arc::new(Mutex::new(10i32));"] # [doc = " let guard = mutex.lock_arc().await;"] # [doc = " dbg!(MutexGuardArc::source(&guard));"] # [doc = " # })"] # [doc = " ```"] pub fn source (guard : & Self) -> & Arc < Mutex < T > > where T : Send , { & guard . 0 } }
};
}
