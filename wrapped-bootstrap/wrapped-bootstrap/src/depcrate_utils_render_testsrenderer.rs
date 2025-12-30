// Generated macro for Renderer (struct)
macro_rules! Depcrate_utils_render_testsRenderer {
() => {
// Module: crate::utils::render_tests
// Provides: {"Renderer"}
// Dependencies: {}
struct Renderer < 'a > { stdout : BufReader < ChildStdout > , failures : Vec < TestOutcome > , benches : Vec < BenchOutcome > , builder : & 'a Builder < 'a > , tests_count : Option < usize > , executed_tests : usize , # [doc = " Number of tests that were skipped due to already being up-to-date"] # [doc = " (i.e. no relevant changes occurred since they last ran)."] up_to_date_tests : usize , ignored_tests : usize , terse_tests_in_line : usize , ci_latest_logged_percentage : f64 , }
};
}
