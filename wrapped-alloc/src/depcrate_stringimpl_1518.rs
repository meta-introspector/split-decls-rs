// Generated macro for impl_1518 (impl)
macro_rules! Depcrate_stringimpl_1518 {
() => {
// Module: crate::string
// Provides: {"impl_1518"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "from_char_for_string" , since = "1.46.0")] impl From < char > for String { # [doc = " Allocates an owned [`String`] from a single character."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " let c: char = 'a';"] # [doc = " let s: String = String::from(c);"] # [doc = " assert_eq!(\"a\", &s[..]);"] # [doc = " ```"] # [inline] fn from (c : char) -> Self { c . to_string () } }
};
}
