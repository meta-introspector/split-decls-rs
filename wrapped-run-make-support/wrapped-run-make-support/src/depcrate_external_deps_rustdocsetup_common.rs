// Generated macro for setup_common (function)
macro_rules! Depcrate_external_deps_rustdocsetup_common {
() => {
// Module: crate::external_deps::rustdoc
// Provides: {"setup_common"}
// Dependencies: {}
# [track_caller] fn setup_common () -> Command { let rustdoc = env_var ("RUSTDOC") ; let mut cmd = Command :: new (rustdoc) ; set_host_compiler_dylib_path (& mut cmd) ; cmd }
};
}
