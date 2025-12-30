// Generated macro for panic_msg_1 (function)
macro_rules! Depcrate_testspanic_msg_1 {
() => {
// Module: crate::tests
// Provides: {"panic_msg_1"}
// Dependencies: {}
# [test] # [should_panic (expected = "foo")] fn panic_msg_1 () { fn prop () -> bool { panic ! ("foo") ; } quickcheck (prop as fn () -> bool) ; }
};
}
