// Generated macro for impl_82 (impl)
macro_rules! Depcrate_remuteximpl_82 {
() => {
// Module: crate::remutex
// Provides: {"impl_82"}
// Dependencies: {}
impl < R , G , T > ReentrantMutex < R , G , T > { # [doc = " Creates a new reentrant mutex based on a pre-existing raw mutex and a"] # [doc = " helper to get the thread ID."] # [inline] pub const fn from_raw (raw_mutex : R , get_thread_id : G , val : T) -> ReentrantMutex < R , G , T > { ReentrantMutex { data : UnsafeCell :: new (val) , raw : RawReentrantMutex { owner : AtomicUsize :: new (0) , lock_count : Cell :: new (0) , mutex : raw_mutex , get_thread_id , } , } } # [doc = " Creates a new reentrant mutex based on a pre-existing raw mutex and a"] # [doc = " helper to get the thread ID."] # [doc = ""] # [doc = " This allows creating a reentrant mutex in a constant context on stable"] # [doc = " Rust."] # [doc = ""] # [doc = " This method is a legacy alias for [`from_raw`](Self::from_raw)."] # [inline] pub const fn const_new (raw_mutex : R , get_thread_id : G , val : T) -> ReentrantMutex < R , G , T > { Self :: from_raw (raw_mutex , get_thread_id , val) } }
};
}
