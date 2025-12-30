// Generated macro for cargo (function)
macro_rules! Depcrate_external_deps_cargocargo {
() => {
// Module: crate::external_deps::cargo
// Provides: {"cargo"}
// Dependencies: {}
# [doc = " Returns a command that can be used to invoke in-tree cargo. The cargo is provided by compiletest"] # [doc = " through the `CARGO` env var, and is **only** available for the `run-make-cargo` test suite."] pub fn cargo () -> Command { let cargo_path = std :: env :: var ("CARGO") . unwrap_or_else (| e | { panic ! ("in-tree `cargo` should be available for `run-make-cargo` test suite, but not \
            `run-make` test suite: {e}") }) ; let mut cmd = Command :: new (cargo_path) ; set_host_compiler_dylib_path (& mut cmd) ; cmd }
};
}
