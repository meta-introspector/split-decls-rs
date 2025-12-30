// Generated macro for run_test (function)
macro_rules! Depcrate_transforms_multi_value_testsrun_test {
() => {
// Module: crate::transforms::multi_value::tests
// Provides: {"run_test"}
// Dependencies: {}
# [rstest :: rstest] fn run_test (# [base_dir = "src/transforms/multi_value/tests"] # [files ("*.wat")] test : PathBuf ,) -> Result < () > { let expected = Test :: from_file (& test) ? ; let actual = runtest (& expected) ? ; expected . check (& actual) }
};
}
