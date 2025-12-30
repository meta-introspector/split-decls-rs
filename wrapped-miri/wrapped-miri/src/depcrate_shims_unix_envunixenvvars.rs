// Generated macro for UnixEnvVars (struct)
macro_rules! Depcrate_shims_unix_envUnixEnvVars {
() => {
// Module: crate::shims::unix::env
// Provides: {"UnixEnvVars"}
// Dependencies: {}
pub struct UnixEnvVars < 'tcx > { # [doc = " Stores pointers to the environment variables. These variables must be stored as"] # [doc = " null-terminated target strings (c_str or wide_str) with the `\"{name}={value}\"` format."] map : FxHashMap < OsString , Pointer > , # [doc = " Place where the `environ` static is stored. Lazily initialized, but then never changes."] environ : MPlaceTy < 'tcx > , }
};
}
