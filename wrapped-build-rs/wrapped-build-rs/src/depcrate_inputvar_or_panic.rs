// Generated macro for var_or_panic (function)
macro_rules! Depcrate_inputvar_or_panic {
() => {
// Module: crate::input
// Provides: {"var_or_panic"}
// Dependencies: {}
# [track_caller] fn var_or_panic (key : & str) -> std :: ffi :: OsString { ENV . get (key) . unwrap_or_else (| | panic ! ("cargo environment variable `{key}` is missing")) }
};
}
