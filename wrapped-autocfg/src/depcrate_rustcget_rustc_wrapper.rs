// Generated macro for get_rustc_wrapper (function)
macro_rules! Depcrate_rustcget_rustc_wrapper {
() => {
// Module: crate::rustc
// Provides: {"get_rustc_wrapper"}
// Dependencies: {}
fn get_rustc_wrapper (workspace : bool) -> Option < PathBuf > { if workspace && env :: var_os ("CARGO_ENCODED_RUSTFLAGS") . is_none () { return None ; } let name = if workspace { "RUSTC_WORKSPACE_WRAPPER" } else { "RUSTC_WRAPPER" } ; if let Some (wrapper) = env :: var_os (name) { if wrapper != OsString :: new () { return Some (wrapper . into ()) ; } } None }
};
}
