// Generated macro for constant_time_eq_n (function)
macro_rules! Depcrate_sse2constant_time_eq_n {
() => {
// Module: crate::sse2
// Provides: {"constant_time_eq_n"}
// Dependencies: {}
# [must_use] pub fn constant_time_eq_n < const N : usize > (a : & [u8 ; N] , b : & [u8 ; N]) -> bool { with_dit (| | constant_time_eq_sse2 (& a [..] , & b [..])) }
};
}
