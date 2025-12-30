// Generated macro for impl_1498 (impl)
macro_rules! Depcrate_stringimpl_1498 {
() => {
// Module: crate::string
// Provides: {"impl_1498"}
// Dependencies: {}
# [stable (feature = "from_string_for_vec_u8" , since = "1.14.0")] impl From < String > for Vec < u8 > { # [doc = " Converts the given [`String`] to a vector [`Vec`] that holds values of type [`u8`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let s1 = String::from(\"hello world\");"] # [doc = " let v1 = Vec::from(s1);"] # [doc = ""] # [doc = " for b in v1 {"] # [doc = "     println!(\"{b}\");"] # [doc = " }"] # [doc = " ```"] fn from (string : String) -> Vec < u8 > { string . into_bytes () } }
};
}
