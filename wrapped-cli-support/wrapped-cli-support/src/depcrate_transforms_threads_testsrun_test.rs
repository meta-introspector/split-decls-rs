// Generated macro for run_test (function)
macro_rules! Depcrate_transforms_threads_testsrun_test {
() => {
// Module: crate::transforms::threads::tests
// Provides: {"run_test"}
// Dependencies: {}
# [rstest :: rstest] fn run_test (# [base_dir = "src/transforms/threads/tests"] # [files ("*.wat")] test : PathBuf ,) -> Result < () > { let expected = Test :: from_file (& test) ? ; let actual = runtest (& expected) ? ; expected . check (& actual) }
};
}
