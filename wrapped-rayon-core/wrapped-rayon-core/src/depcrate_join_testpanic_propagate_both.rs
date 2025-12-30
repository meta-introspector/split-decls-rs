// Generated macro for panic_propagate_both (function)
macro_rules! Depcrate_join_testpanic_propagate_both {
() => {
// Module: crate::join::test
// Provides: {"panic_propagate_both"}
// Dependencies: {}
# [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_both () { join (| | panic ! ("Hello, world!") , | | panic ! ("Goodbye, world!")) ; }
};
}
