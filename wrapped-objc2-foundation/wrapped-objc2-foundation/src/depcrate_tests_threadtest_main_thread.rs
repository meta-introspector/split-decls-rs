// Generated macro for test_main_thread (function)
macro_rules! Depcrate_tests_threadtest_main_thread {
() => {
// Module: crate::tests::thread
// Provides: {"test_main_thread"}
// Dependencies: {}
# [test] # [cfg_attr (feature = "gnustep-1-7" , ignore = "Retrieving main thread is weirdly broken, only works with --test-threads=1")] fn test_main_thread () { let current = NSThread :: currentThread () ; let main = NSThread :: mainThread () ; assert ! (main . isMainThread ()) ; if main == current { assert ! (current . isMainThread ()) ; assert ! (MainThreadMarker :: new () . is_some ()) ; } else { assert ! (! current . isMainThread ()) ; assert ! (MainThreadMarker :: new () . is_none ()) ; } }
};
}
