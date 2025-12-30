// Generated macro for expand_args (function)
macro_rules! Depcrate_expandexpand_args {
() => {
// Module: crate::expand
// Provides: {"expand_args"}
// Dependencies: {}
# [doc = " Same as [`expand`] but allows to pass additional arguments to `cargo-expand`."] # [doc = ""] # [doc = " [`expand`]: expand/fn.expand.html"] pub fn expand_args < I , S > (path : impl AsRef < Path > , args : I) where I : IntoIterator < Item = S > + Clone , S : AsRef < OsStr > , { run_tests (path , ExpansionBehavior :: RegenerateFiles , Some (args)) ; }
};
}
