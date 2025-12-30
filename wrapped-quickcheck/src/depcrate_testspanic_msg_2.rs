// Generated macro for panic_msg_2 (function)
macro_rules! Depcrate_testspanic_msg_2 {
() => {
// Module: crate::tests
// Provides: {"panic_msg_2"}
// Dependencies: {}
# [test] # [should_panic (expected = "foo")] fn panic_msg_2 () { fn prop () -> bool { assert ! ("foo" == "bar") ; true } quickcheck (prop as fn () -> bool) ; }
};
}
