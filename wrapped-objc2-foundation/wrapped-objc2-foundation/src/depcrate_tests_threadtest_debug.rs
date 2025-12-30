// Generated macro for test_debug (function)
macro_rules! Depcrate_tests_threadtest_debug {
() => {
// Module: crate::tests::thread
// Provides: {"test_debug"}
// Dependencies: {}
# [test] # [cfg_attr (feature = "gnustep-1-7" , ignore = "Retrieving main thread is weirdly broken, only works with --test-threads=1")] fn test_debug () { let thread = NSThread :: mainThread () ; let actual = format ! ("{thread:?}") ; let expected = [format ! ("<NSThread: {thread:p}>{{number = 1, name = (null)}}") , format ! ("<NSThread: {thread:p}>{{number = 1, name = main}}") , format ! ("<_NSMainThread: {thread:p}>{{number = 1, name = (null)}}") , format ! ("<_NSMainThread: {thread:p}>{{number = 1, name = main}}") ,] ; assert ! (expected . contains (& actual) , "Expected one of {expected:?}, got {actual:?}" ,) ; }
};
}
