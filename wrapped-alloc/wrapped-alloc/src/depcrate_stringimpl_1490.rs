// Generated macro for impl_1490 (impl)
macro_rules! Depcrate_stringimpl_1490 {
() => {
// Module: crate::string
// Provides: {"impl_1490"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "box_from_str" , since = "1.20.0")] impl From < String > for Box < str > { # [doc = " Converts the given [`String`] to a boxed `str` slice that is owned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let s1: String = String::from(\"hello world\");"] # [doc = " let s2: Box<str> = Box::from(s1);"] # [doc = " let s3: String = String::from(s2);"] # [doc = ""] # [doc = " assert_eq!(\"hello world\", s3)"] # [doc = " ```"] fn from (s : String) -> Box < str > { s . into_boxed_str () } }
};
}
