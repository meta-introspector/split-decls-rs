// Generated macro for constant_time_ne (function)
macro_rules! Depcrate_classicconstant_time_ne {
() => {
// Module: crate::classic
// Provides: {"constant_time_ne"}
// Dependencies: {}
# [inline] # [must_use] fn constant_time_ne (a : & [u8] , b : & [u8]) -> u8 { assert ! (a . len () == b . len ()) ; let len = a . len () ; let a = & a [.. len] ; let b = & b [.. len] ; let mut tmp = 0 ; for i in 0 .. len { tmp |= a [i] ^ b [i] ; } optimizer_hide (tmp) }
};
}
