// Generated macro for inject_cargo_env (function)
macro_rules! Depcrate_envinject_cargo_env {
() => {
// Module: crate::env
// Provides: {"inject_cargo_env"}
// Dependencies: {}
pub (crate) fn inject_cargo_env (env : & mut Env , cargo_path : & Utf8Path) { env . set ("CARGO" , cargo_path . as_str ()) ; }
};
}
