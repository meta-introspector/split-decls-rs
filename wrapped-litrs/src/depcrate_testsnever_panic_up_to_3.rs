// Generated macro for never_panic_up_to_3 (function)
macro_rules! Depcrate_testsnever_panic_up_to_3 {
() => {
// Module: crate::tests
// Provides: {"never_panic_up_to_3"}
// Dependencies: {}
# [test] # [ignore] fn never_panic_up_to_3 () { for a in 0 .. 128 { assert_no_panic ! ([a]) ; for b in 0 .. 128 { assert_no_panic ! ([a , b]) ; for c in 0 .. 128 { assert_no_panic ! ([a , b , c]) ; } } } }
};
}
