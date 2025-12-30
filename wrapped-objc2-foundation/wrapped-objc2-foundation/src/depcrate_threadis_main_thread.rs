// Generated macro for is_main_thread (function)
macro_rules! Depcrate_threadis_main_thread {
() => {
// Module: crate::thread
// Provides: {"is_main_thread"}
// Dependencies: {}
# [doc = " Whether the current thread is the main thread."] # [doc = ""] # [doc = " Deprecated. Prefer `MainThreadMarker::new().is_some()` or"] # [doc = " `NSThread::isMainThread_class()` instead."] # [deprecated = "use `objc2::MainThreadMarker::new().is_some()`"] pub fn is_main_thread () -> bool { MainThreadMarker :: new () . is_some () }
};
}
