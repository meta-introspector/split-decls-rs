// Generated macro for constant_time_eq (function)
macro_rules! Depcrate_classicconstant_time_eq {
() => {
// Module: crate::classic
// Provides: {"constant_time_eq"}
// Dependencies: {}
# [doc = " Compares two equal-sized byte strings in constant time."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use constant_time_eq::constant_time_eq;"] # [doc = ""] # [doc = " assert!(constant_time_eq(b\"foo\", b\"foo\"));"] # [doc = " assert!(!constant_time_eq(b\"foo\", b\"bar\"));"] # [doc = " assert!(!constant_time_eq(b\"bar\", b\"baz\"));"] # [doc = " # assert!(constant_time_eq(b\"\", b\"\"));"] # [doc = ""] # [doc = " // Not equal-sized, so won't take constant time."] # [doc = " assert!(!constant_time_eq(b\"foo\", b\"\"));"] # [doc = " assert!(!constant_time_eq(b\"foo\", b\"quux\"));"] # [doc = " ```"] # [must_use] pub fn constant_time_eq (a : & [u8] , b : & [u8]) -> bool { with_dit (| | a . len () == b . len () && constant_time_ne (a , b) == 0) }
};
}
