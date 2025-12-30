// Generated macro for check_repeat_unbounded (function)
macro_rules! Depcrate_iter_testcheck_repeat_unbounded {
() => {
// Module: crate::iter::test
// Provides: {"check_repeat_unbounded"}
// Dependencies: {}
# [test] # [ignore] # [should_panic (expected = "overflow")] # [cfg (debug_assertions)] fn check_repeat_unbounded () { let pool = ThreadPoolBuilder :: new () . num_threads (1) . build () . unwrap () ; pool . install (| | { println ! ("counted {} repeats" , repeat (()) . count ()) ; }) ; }
};
}
