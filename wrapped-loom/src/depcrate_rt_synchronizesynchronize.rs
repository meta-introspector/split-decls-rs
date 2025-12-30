// Generated macro for Synchronize (struct)
macro_rules! Depcrate_rt_synchronizeSynchronize {
() => {
// Module: crate::rt::synchronize
// Provides: {"Synchronize"}
// Dependencies: {}
# [doc = " A synchronization point between two threads."] # [doc = ""] # [doc = " Threads synchronize with this point using any of the available orderings. On"] # [doc = " loads, the thread's causality is updated using the synchronization point's"] # [doc = " stored causality. On stores, the synchronization point's causality is"] # [doc = " updated with the threads."] # [derive (Debug , Clone , Copy)] pub (crate) struct Synchronize { happens_before : VersionVec , }
};
}
