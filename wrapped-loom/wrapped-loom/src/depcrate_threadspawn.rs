// Generated macro for spawn (function)
macro_rules! Depcrate_threadspawn {
() => {
// Module: crate::thread
// Provides: {"spawn"}
// Dependencies: {}
# [doc = " Mock implementation of `std::thread::spawn`."] # [doc = ""] # [doc = " Note that you may only have [`MAX_THREADS`](crate::MAX_THREADS) threads in a given loom tests"] # [doc = " _including_ the main thread."] # [track_caller] pub fn spawn < F , T > (f : F) -> JoinHandle < T > where F : FnOnce () -> T , F : 'static , T : 'static , { spawn_internal (f , None , None , location ! ()) }
};
}
