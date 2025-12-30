// Generated macro for PARKED_BIT (const)
macro_rules! Depcrate_raw_mutexPARKED_BIT {
() => {
// Module: crate::raw_mutex
// Provides: {"PARKED_BIT"}
// Dependencies: {}
# [doc = " This bit is set in the `state` of a `RawMutex` just before parking a thread. A thread is being"] # [doc = " parked if it wants to lock the mutex, but it is currently being held by some other thread."] const PARKED_BIT : u8 = 0b10 ;
};
}
