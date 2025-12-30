// Generated macro for panic_propagate (function)
macro_rules! Depcrate_thread_pool_testpanic_propagate {
() => {
// Module: crate::thread_pool::test
// Provides: {"panic_propagate"}
// Dependencies: {}
# [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate () { let thread_pool = ThreadPoolBuilder :: new () . build () . unwrap () ; thread_pool . install (| | { panic ! ("Hello, world!") ; }) ; }
};
}
