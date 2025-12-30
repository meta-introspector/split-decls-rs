// Generated macro for impl_62 (impl)
macro_rules! Depcrate_muteximpl_62 {
() => {
// Module: crate::mutex
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'a , T : ? Sized > MutexGuard < 'a , T > { # [doc = " Returns a reference to the mutex a guard came from."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use async_lock::{Mutex, MutexGuard};"] # [doc = ""] # [doc = " let mutex = Mutex::new(10i32);"] # [doc = " let guard = mutex.lock().await;"] # [doc = " dbg!(MutexGuard::source(&guard));"] # [doc = " # })"] # [doc = " ```"] pub fn source (guard : & MutexGuard < 'a , T >) -> & 'a Mutex < T > { guard . 0 } }
};
}
