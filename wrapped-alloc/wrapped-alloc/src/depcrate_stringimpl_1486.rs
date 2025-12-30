// Generated macro for impl_1486 (impl)
macro_rules! Depcrate_stringimpl_1486 {
() => {
// Module: crate::string
// Provides: {"impl_1486"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl From < & str > for String { # [doc = " Converts a `&str` into a [`String`]."] # [doc = ""] # [doc = " The result is allocated on the heap."] # [inline] fn from (s : & str) -> String { s . to_owned () } }
};
}
