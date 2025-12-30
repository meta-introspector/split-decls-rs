// Generated macro for expand_without_refresh_args (function)
macro_rules! Depcrate_expandexpand_without_refresh_args {
() => {
// Module: crate::expand
// Provides: {"expand_without_refresh_args"}
// Dependencies: {}
# [doc = " Same as [`expand_without_refresh`] but allows to pass additional arguments to `cargo-expand`."] # [doc = ""] # [doc = " [`expand_without_refresh`]: expand/fn.expand_without_refresh.html"] pub fn expand_without_refresh_args < I , S > (path : impl AsRef < Path > , args : I) where I : IntoIterator < Item = S > + Clone , S : AsRef < OsStr > , { run_tests (path , ExpansionBehavior :: ExpectFiles , Some (args)) ; }
};
}
