// Generated macro for const_fair_mutex (function)
macro_rules! Depcrate_fair_mutexconst_fair_mutex {
() => {
// Module: crate::fair_mutex
// Provides: {"const_fair_mutex"}
// Dependencies: {}
# [doc = " Creates a new fair mutex in an unlocked state ready for use."] # [doc = ""] # [doc = " This allows creating a fair mutex in a constant context on stable Rust."] pub const fn const_fair_mutex < T > (val : T) -> FairMutex < T > { FairMutex :: const_new (< RawFairMutex as lock_api :: RawMutex > :: INIT , val) }
};
}
