// Generated macro for const_mutex (function)
macro_rules! Depcrate_mutexconst_mutex {
() => {
// Module: crate::mutex
// Provides: {"const_mutex"}
// Dependencies: {}
# [doc = " Creates a new mutex in an unlocked state ready for use."] # [doc = ""] # [doc = " This allows creating a mutex in a constant context on stable Rust."] pub const fn const_mutex < T > (val : T) -> Mutex < T > { Mutex :: const_new (< RawMutex as lock_api :: RawMutex > :: INIT , val) }
};
}
