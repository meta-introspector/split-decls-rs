// Generated macro for rt_threaded (function)
macro_rules! Depcrate_testsrt_threaded {
() => {
// Module: crate::tests
// Provides: {"rt_threaded"}
// Dependencies: {}
fn rt_threaded () -> Runtime { Builder :: new_multi_thread () . enable_all () . build () . unwrap () }
};
}
