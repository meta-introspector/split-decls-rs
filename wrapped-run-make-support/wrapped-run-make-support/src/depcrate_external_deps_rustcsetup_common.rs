// Generated macro for setup_common (function)
macro_rules! Depcrate_external_deps_rustcsetup_common {
() => {
// Module: crate::external_deps::rustc
// Provides: {"setup_common"}
// Dependencies: {}
# [track_caller] fn setup_common () -> Command { let mut cmd = Command :: new (rustc_path ()) ; set_host_compiler_dylib_path (& mut cmd) ; cmd }
};
}
