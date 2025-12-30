// Generated macro for panic_msg_3 (function)
macro_rules! Depcrate_testspanic_msg_3 {
() => {
// Module: crate::tests
// Provides: {"panic_msg_3"}
// Dependencies: {}
# [test] # [should_panic (expected = "foo")] fn panic_msg_3 () { fn prop () -> bool { assert_eq ! ("foo" , "bar") ; true } quickcheck (prop as fn () -> bool) ; }
};
}
