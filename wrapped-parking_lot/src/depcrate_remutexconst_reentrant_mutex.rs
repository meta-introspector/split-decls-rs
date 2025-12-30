// Generated macro for const_reentrant_mutex (function)
macro_rules! Depcrate_remutexconst_reentrant_mutex {
() => {
// Module: crate::remutex
// Provides: {"const_reentrant_mutex"}
// Dependencies: {}
# [doc = " Creates a new reentrant mutex in an unlocked state ready for use."] # [doc = ""] # [doc = " This allows creating a reentrant mutex in a constant context on stable Rust."] pub const fn const_reentrant_mutex < T > (val : T) -> ReentrantMutex < T > { ReentrantMutex :: const_new (< RawMutex as lock_api :: RawMutex > :: INIT , < RawThreadId as lock_api :: GetThreadId > :: INIT , val ,) }
};
}
