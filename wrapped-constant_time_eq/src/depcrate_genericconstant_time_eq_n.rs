// Generated macro for constant_time_eq_n (function)
macro_rules! Depcrate_genericconstant_time_eq_n {
() => {
// Module: crate::generic
// Provides: {"constant_time_eq_n"}
// Dependencies: {}
# [doc = " Compares two fixed-size byte strings in constant time."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use constant_time_eq::constant_time_eq_n;"] # [doc = ""] # [doc = " assert!(constant_time_eq_n(&[3; 20], &[3; 20]));"] # [doc = " assert!(!constant_time_eq_n(&[3; 20], &[7; 20]));"] # [doc = " ```"] # [must_use] pub fn constant_time_eq_n < const N : usize > (a : & [u8 ; N] , b : & [u8 ; N]) -> bool { with_dit (| | constant_time_eq_impl (& a [..] , & b [..] , 0)) }
};
}
