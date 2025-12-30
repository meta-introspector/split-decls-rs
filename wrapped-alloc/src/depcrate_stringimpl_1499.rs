// Generated macro for impl_1499 (impl)
macro_rules! Depcrate_stringimpl_1499 {
() => {
// Module: crate::string
// Provides: {"impl_1499"}
// Dependencies: {}
# [stable (feature = "try_from_vec_u8_for_string" , since = "1.87.0")] impl TryFrom < Vec < u8 > > for String { type Error = FromUtf8Error ; # [doc = " Converts the given [`Vec<u8>`] into a  [`String`] if it contains valid UTF-8 data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let s1 = b\"hello world\".to_vec();"] # [doc = " let v1 = String::try_from(s1).unwrap();"] # [doc = " assert_eq!(v1, \"hello world\");"] # [doc = ""] # [doc = " ```"] fn try_from (bytes : Vec < u8 >) -> Result < Self , Self :: Error > { Self :: from_utf8 (bytes) } }
};
}
