// Generated macro for copy (function)
macro_rules! Depcrate_pathscopy {
() => {
// Module: crate::paths
// Provides: {"copy"}
// Dependencies: {}
# [doc = " Copies a file from one location to another."] # [doc = ""] # [doc = " Equivalent to [`std::fs::copy`] with better error messages."] pub fn copy < P : AsRef < Path > , Q : AsRef < Path > > (from : P , to : Q) -> Result < u64 > { let from = from . as_ref () ; let to = to . as_ref () ; fs :: copy (from , to) . with_context (| | format ! ("failed to copy `{}` to `{}`" , from . display () , to . display ())) }
};
}
