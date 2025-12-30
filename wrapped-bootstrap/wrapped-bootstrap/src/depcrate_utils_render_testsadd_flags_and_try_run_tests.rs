// Generated macro for add_flags_and_try_run_tests (function)
macro_rules! Depcrate_utils_render_testsadd_flags_and_try_run_tests {
() => {
// Module: crate::utils::render_tests
// Provides: {"add_flags_and_try_run_tests"}
// Dependencies: {}
pub (crate) fn add_flags_and_try_run_tests (builder : & Builder < '_ > , cmd : & mut BootstrapCommand ,) -> bool { if ! cmd . get_args () . any (| arg | arg == "--") { cmd . arg ("--") ; } cmd . args (["-Z" , "unstable-options" , "--format" , "json"]) ; try_run_tests (builder , cmd , false) }
};
}
