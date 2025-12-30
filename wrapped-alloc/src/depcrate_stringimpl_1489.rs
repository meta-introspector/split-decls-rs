// Generated macro for impl_1489 (impl)
macro_rules! Depcrate_stringimpl_1489 {
() => {
// Module: crate::string
// Provides: {"impl_1489"}
// Dependencies: {}
# [stable (feature = "string_from_box" , since = "1.18.0")] impl From < Box < str > > for String { # [doc = " Converts the given boxed `str` slice to a [`String`]."] # [doc = " It is notable that the `str` slice is owned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let s1: String = String::from(\"hello world\");"] # [doc = " let s2: Box<str> = s1.into_boxed_str();"] # [doc = " let s3: String = String::from(s2);"] # [doc = ""] # [doc = " assert_eq!(\"hello world\", s3)"] # [doc = " ```"] fn from (s : Box < str >) -> String { s . into_string () } }
};
}
