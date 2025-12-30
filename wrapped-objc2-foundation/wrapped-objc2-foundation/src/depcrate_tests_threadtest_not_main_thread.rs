// Generated macro for test_not_main_thread (function)
macro_rules! Depcrate_tests_threadtest_not_main_thread {
() => {
// Module: crate::tests::thread
// Provides: {"test_not_main_thread"}
// Dependencies: {}
# [test] # [cfg (feature = "std")] fn test_not_main_thread () { let res = std :: thread :: spawn (| | NSThread :: currentThread () . isMainThread ()) . join () . unwrap () ; assert ! (! res) ; }
};
}
