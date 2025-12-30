// Generated macro for panic_propagate_scope (function)
macro_rules! Depcrate_scope_testpanic_propagate_scope {
() => {
// Module: crate::scope::test
// Provides: {"panic_propagate_scope"}
// Dependencies: {}
# [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_scope () { scope (| _ | panic ! ("Hello, world!")) ; }
};
}
