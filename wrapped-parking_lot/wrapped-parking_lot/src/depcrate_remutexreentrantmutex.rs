// Generated macro for ReentrantMutex (type)
macro_rules! Depcrate_remutexReentrantMutex {
() => {
// Module: crate::remutex
// Provides: {"ReentrantMutex"}
// Dependencies: {}
# [doc = " A mutex which can be recursively locked by a single thread."] # [doc = ""] # [doc = " This type is identical to `Mutex` except for the following points:"] # [doc = ""] # [doc = " - Locking multiple times from the same thread will work correctly instead of"] # [doc = "   deadlocking."] # [doc = " - `ReentrantMutexGuard` does not give mutable references to the locked data."] # [doc = "   Use a `RefCell` if you need this."] # [doc = ""] # [doc = " See [`Mutex`](crate::Mutex) for more details about the underlying mutex"] # [doc = " primitive."] pub type ReentrantMutex < T > = lock_api :: ReentrantMutex < RawMutex , RawThreadId , T > ;
};
}
