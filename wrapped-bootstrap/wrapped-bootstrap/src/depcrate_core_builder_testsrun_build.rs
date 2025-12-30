// Generated macro for run_build (function)
macro_rules! Depcrate_core_builder_testsrun_build {
() => {
// Module: crate::core::builder::tests
// Provides: {"run_build"}
// Dependencies: {}
fn run_build (paths : & [PathBuf] , config : Config) -> Cache { let kind = config . cmd . kind () ; let build = Build :: new (config) ; let builder = Builder :: new (& build) ; builder . run_step_descriptions (& Builder :: get_step_descriptions (kind) , paths) ; builder . cache }
};
}
