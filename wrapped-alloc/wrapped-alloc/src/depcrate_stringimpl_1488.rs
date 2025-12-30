// Generated macro for impl_1488 (impl)
macro_rules! Depcrate_stringimpl_1488 {
() => {
// Module: crate::string
// Provides: {"impl_1488"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "from_ref_string" , since = "1.35.0")] impl From < & String > for String { # [doc = " Converts a `&String` into a [`String`]."] # [doc = ""] # [doc = " This clones `s` and returns the clone."] # [inline] fn from (s : & String) -> String { s . clone () } }
};
}
