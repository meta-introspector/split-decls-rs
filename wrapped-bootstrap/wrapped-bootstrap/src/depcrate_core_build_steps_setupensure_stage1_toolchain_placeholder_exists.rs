// Generated macro for ensure_stage1_toolchain_placeholder_exists (function)
macro_rules! Depcrate_core_build_steps_setupensure_stage1_toolchain_placeholder_exists {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"ensure_stage1_toolchain_placeholder_exists"}
// Dependencies: {}
fn ensure_stage1_toolchain_placeholder_exists (stage_path : & str) -> bool { let pathbuf = PathBuf :: from (stage_path) ; if fs :: create_dir_all (pathbuf . join ("lib")) . is_err () { return false ; } ; let pathbuf = pathbuf . join ("bin") ; if fs :: create_dir_all (& pathbuf) . is_err () { return false ; } ; let pathbuf = pathbuf . join (format ! ("rustc{EXE_SUFFIX}")) ; if pathbuf . exists () { return true ; } let result = File :: options () . append (true) . create (true) . open (& pathbuf) ; if result . is_err () { return false ; } true }
};
}
