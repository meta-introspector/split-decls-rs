// Generated macro for coverage_args (function)
macro_rules! Depcrate_wasm_bindgen_test_runnercoverage_args {
() => {
// Module: crate::wasm_bindgen_test_runner
// Provides: {"coverage_args"}
// Dependencies: {}
fn coverage_args (file_name : & Path) -> PathBuf { fn generated (file_name : & Path , prefix : & str) -> String { let res = format ! ("{prefix}{}.profraw" , file_name . display ()) ; res } let prefix = env :: var_os ("WASM_BINDGEN_UNSTABLE_TEST_PROFRAW_PREFIX") . map (| s | s . to_str () . unwrap () . to_string ()) . unwrap_or_default () ; match env :: var_os ("WASM_BINDGEN_UNSTABLE_TEST_PROFRAW_OUT") { Some (s) => { let mut buf = PathBuf :: from (s) ; if buf . is_dir () { buf . push (generated (file_name , & prefix)) ; } buf } None => PathBuf :: from (generated (file_name , & prefix)) , } }
};
}
