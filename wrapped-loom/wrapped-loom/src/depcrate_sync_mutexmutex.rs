// Generated macro for Mutex (struct)
macro_rules! Depcrate_sync_mutexMutex {
() => {
// Module: crate::sync::mutex
// Provides: {"Mutex"}
// Dependencies: {}
# [doc = " Mock implementation of `std::sync::Mutex`."] # [derive (Debug)] pub struct Mutex < T : ? Sized > { object : rt :: Mutex , data : std :: sync :: Mutex < T > , }
};
}
