// Generated macro for Mutex (struct)
macro_rules! Depcrate_mutexMutex {
() => {
// Module: crate::mutex
// Provides: {"Mutex"}
// Dependencies: {}
# [doc = " An async mutex."] # [doc = ""] # [doc = " The locking mechanism uses eventual fairness to ensure locking will be fair on average without"] # [doc = " sacrificing performance. This is done by forcing a fair lock whenever a lock operation is"] # [doc = " starved for longer than 0.5 milliseconds."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use async_lock::Mutex;"] # [doc = ""] # [doc = " let m = Mutex::new(1);"] # [doc = ""] # [doc = " let mut guard = m.lock().await;"] # [doc = " *guard = 2;"] # [doc = ""] # [doc = " assert!(m.try_lock().is_none());"] # [doc = " drop(guard);"] # [doc = " assert_eq!(*m.try_lock().unwrap(), 2);"] # [doc = " # })"] # [doc = " ```"] pub struct Mutex < T : ? Sized > { # [doc = " Current state of the mutex."] # [doc = ""] # [doc = " The least significant bit is set to 1 if the mutex is locked."] # [doc = " The other bits hold the number of starved lock operations."] state : AtomicUsize , # [doc = " Lock operations waiting for the mutex to be released."] lock_ops : Event , # [doc = " The value inside the mutex."] data : UnsafeCell < T > , }
};
}
