// Generated macro for thread_indices (function)
macro_rules! Depcrate_sync_sharded_lockthread_indices {
() => {
// Module: crate::sync::sharded_lock
// Provides: {"thread_indices"}
// Dependencies: {}
fn thread_indices () -> & 'static Mutex < ThreadIndices > { static THREAD_INDICES : OnceLock < Mutex < ThreadIndices > > = OnceLock :: new () ; fn init () -> Mutex < ThreadIndices > { Mutex :: new (ThreadIndices { mapping : HashMap :: new () , free_list : Vec :: new () , next_index : 0 , }) } THREAD_INDICES . get_or_init (init) }
};
}
