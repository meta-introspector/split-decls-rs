// Generated macro for setup_rustc_src (function)
macro_rules! Depcrate_setup_intellijsetup_rustc_src {
() => {
// Module: crate::setup::intellij
// Provides: {"setup_rustc_src"}
// Dependencies: {}
pub fn setup_rustc_src (rustc_path : & str) { let Ok (rustc_source_dir) = check_and_get_rustc_dir (rustc_path) else { return ; } ; for project in CLIPPY_PROJECTS { if inject_deps_into_project (& rustc_source_dir , project) . is_err () { return ; } } println ! ("info: the source paths can be removed again with `cargo dev remove intellij`") ; }
};
}
