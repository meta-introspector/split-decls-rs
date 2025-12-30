// Generated macro for check_cli (function)
macro_rules! Depcrate_core_builder_testscheck_cli {
() => {
// Module: crate::core::builder::tests
// Provides: {"check_cli"}
// Dependencies: {}
fn check_cli < const N : usize > (paths : [& str ; N]) { run_build (& paths . map (PathBuf :: from) , configure_with_args (& paths , & [TEST_TRIPLE_1] , & [TEST_TRIPLE_1]) ,) ; }
};
}
