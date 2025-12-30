// Generated macro for threaded_timeout (function)
macro_rules! Depcratethreaded_timeout {
() => {
// Module: crate
// Provides: {"threaded_timeout"}
// Dependencies: {}
# [doc = " Helper that runs some function, and waits up to `n` tenths of a second for"] # [doc = " it to finish."] # [track_caller] pub fn threaded_timeout < F , R > (n : u32 , f : F) -> R where F : FnOnce () -> R + Send + 'static , R : Send + 'static , { let thread = std :: thread :: spawn (| | f ()) ; thread_wait_timeout (n , thread) }
};
}
