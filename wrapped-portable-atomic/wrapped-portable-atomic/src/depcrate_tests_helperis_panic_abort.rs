// Generated macro for is_panic_abort (function)
macro_rules! Depcrate_tests_helperis_panic_abort {
() => {
// Module: crate::tests::helper
// Provides: {"is_panic_abort"}
// Dependencies: {}
fn is_panic_abort () -> bool { ! matches ! (build_context :: PANIC , "unwind" | "") }
};
}
