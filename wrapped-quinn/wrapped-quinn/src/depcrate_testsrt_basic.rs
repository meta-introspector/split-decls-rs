// Generated macro for rt_basic (function)
macro_rules! Depcrate_testsrt_basic {
() => {
// Module: crate::tests
// Provides: {"rt_basic"}
// Dependencies: {}
fn rt_basic () -> Runtime { Builder :: new_current_thread () . enable_all () . build () . unwrap () }
};
}
