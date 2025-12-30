// Generated macro for const_rwlock (function)
macro_rules! Depcrate_rwlockconst_rwlock {
() => {
// Module: crate::rwlock
// Provides: {"const_rwlock"}
// Dependencies: {}
# [doc = " Creates a new instance of an `RwLock<T>` which is unlocked."] # [doc = ""] # [doc = " This allows creating a `RwLock<T>` in a constant context on stable Rust."] pub const fn const_rwlock < T > (val : T) -> RwLock < T > { RwLock :: const_new (< RawRwLock as lock_api :: RawRwLock > :: INIT , val) }
};
}
