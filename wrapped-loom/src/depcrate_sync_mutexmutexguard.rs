// Generated macro for MutexGuard (struct)
macro_rules! Depcrate_sync_mutexMutexGuard {
() => {
// Module: crate::sync::mutex
// Provides: {"MutexGuard"}
// Dependencies: {}
# [doc = " Mock implementation of `std::sync::MutexGuard`."] # [derive (Debug)] pub struct MutexGuard < 'a , T : ? Sized > { lock : & 'a Mutex < T > , data : Option < std :: sync :: MutexGuard < 'a , T > > , }
};
}
