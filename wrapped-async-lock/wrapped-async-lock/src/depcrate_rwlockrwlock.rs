// Generated macro for RwLock (struct)
macro_rules! Depcrate_rwlockRwLock {
() => {
// Module: crate::rwlock
// Provides: {"RwLock"}
// Dependencies: {}
# [doc = " An async reader-writer lock."] # [doc = ""] # [doc = " This type of lock allows multiple readers or one writer at any point in time."] # [doc = ""] # [doc = " The locking strategy is write-preferring, which means writers are never starved."] # [doc = " Releasing a write lock wakes the next blocked reader and the next blocked writer."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use async_lock::RwLock;"] # [doc = ""] # [doc = " let lock = RwLock::new(5);"] # [doc = ""] # [doc = " // Multiple read locks can be held at a time."] # [doc = " let r1 = lock.read().await;"] # [doc = " let r2 = lock.read().await;"] # [doc = " assert_eq!(*r1, 5);"] # [doc = " assert_eq!(*r2, 5);"] # [doc = " drop((r1, r2));"] # [doc = ""] # [doc = " // Only one write lock can be held at a time."] # [doc = " let mut w = lock.write().await;"] # [doc = " *w += 1;"] # [doc = " assert_eq!(*w, 6);"] # [doc = " # })"] # [doc = " ```"] pub struct RwLock < T : ? Sized > { # [doc = " The underlying locking implementation."] # [doc = " Doesn't depend on `T`."] raw : RawRwLock , # [doc = " The inner value."] value : UnsafeCell < T > , }
};
}
