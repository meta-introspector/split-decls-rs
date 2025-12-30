// Generated macro for test_iter_drain (function)
macro_rules! Depcrate_blinktest_iter_drain {
() => {
// Module: crate::blink
// Provides: {"test_iter_drain"}
// Dependencies: {}
# [test] fn test_iter_drain () { assert_eq ! (5 , saturating_drain_iter (0 .. 5)) ; assert_eq ! (usize :: MAX , saturating_drain_iter (0 .. usize :: MAX)) ; assert_eq ! (usize :: MAX , saturating_drain_iter (core :: iter :: repeat (1))) ; }
};
}
