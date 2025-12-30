// Generated macro for SuiteOutcome (struct)
macro_rules! Depcrate_utils_render_testsSuiteOutcome {
() => {
// Module: crate::utils::render_tests
// Provides: {"SuiteOutcome"}
// Dependencies: {}
# [derive (serde_derive :: Deserialize)] struct SuiteOutcome { passed : usize , failed : usize , ignored : usize , measured : usize , filtered_out : usize , # [doc = " The time it took to execute this test suite, or `None` if time measurement was not possible"] # [doc = " (e.g. due to running on wasm)."] exec_time : Option < f64 > , }
};
}
