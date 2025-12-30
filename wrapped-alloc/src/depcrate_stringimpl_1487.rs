// Generated macro for impl_1487 (impl)
macro_rules! Depcrate_stringimpl_1487 {
() => {
// Module: crate::string
// Provides: {"impl_1487"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "from_mut_str_for_string" , since = "1.44.0")] impl From < & mut str > for String { # [doc = " Converts a `&mut str` into a [`String`]."] # [doc = ""] # [doc = " The result is allocated on the heap."] # [inline] fn from (s : & mut str) -> String { s . to_owned () } }
};
}
