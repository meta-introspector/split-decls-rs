// Generated macro for constant_time_ne_n (function)
macro_rules! Depcrate_classicconstant_time_ne_n {
() => {
// Module: crate::classic
// Provides: {"constant_time_ne_n"}
// Dependencies: {}
# [inline] # [must_use] fn constant_time_ne_n < const N : usize > (a : & [u8 ; N] , b : & [u8 ; N]) -> u8 { let mut tmp = 0 ; for i in 0 .. N { tmp |= a [i] ^ b [i] ; } optimizer_hide (tmp) }
};
}
