// Generated macro for skip_should_panic_test (function)
macro_rules! Depcrate_tests_helperskip_should_panic_test {
() => {
// Module: crate::tests::helper
// Provides: {"skip_should_panic_test"}
// Dependencies: {}
fn skip_should_panic_test () -> bool { is_panic_abort () || cfg ! (miri) || option_env ! ("CARGO_PROFILE_RELEASE_LTO") . map_or (false , | v | v == "fat") && build_context :: SANITIZE . contains ("memory") }
};
}
