// Generated macro for ThreadManager (struct)
macro_rules! Depcrate_concurrency_threadThreadManager {
() => {
// Module: crate::concurrency::thread
// Provides: {"ThreadManager"}
// Dependencies: {}
# [doc = " A set of threads."] # [derive (Debug)] pub struct ThreadManager < 'tcx > { # [doc = " Identifier of the currently active thread."] active_thread : ThreadId , # [doc = " Threads used in the program."] # [doc = ""] # [doc = " Note that this vector also contains terminated threads."] threads : IndexVec < ThreadId , Thread < 'tcx > > , # [doc = " A mapping from a thread-local static to the thread specific allocation."] thread_local_allocs : FxHashMap < (DefId , ThreadId) , StrictPointer > , # [doc = " A flag that indicates that we should change the active thread."] yield_active_thread : bool , # [doc = " A flag that indicates that we should do round robin scheduling of threads else randomized scheduling is used."] fixed_scheduling : bool , }
};
}
