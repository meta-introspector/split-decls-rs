// Generated macro for args_os_opt (function)
macro_rules! Depcrate_envargs_os_opt {
() => {
// Module: crate::env
// Provides: {"args_os_opt"}
// Dependencies: {}
# [doc = " Like [`args_os()`], but with the `precompose_unicode` parameter akin to `core.precomposeUnicode` in the Git configuration."] pub fn args_os_opt (precompose_unicode : bool) -> impl Iterator < Item = OsString > { std :: env :: args_os () . map (move | arg | { if precompose_unicode { gix_utils :: str :: precompose_os_string (arg . into ()) . into_owned () } else { arg } }) }
};
}
