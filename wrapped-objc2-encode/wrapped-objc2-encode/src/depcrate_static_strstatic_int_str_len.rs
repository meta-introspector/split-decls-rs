// Generated macro for static_int_str_len (function)
macro_rules! Depcrate_static_strstatic_int_str_len {
() => {
// Module: crate::static_str
// Provides: {"static_int_str_len"}
// Dependencies: {}
pub (crate) const fn static_int_str_len (mut n : u64) -> usize { let mut i = 0 ; if n == 0 { return 1 ; } while n > 0 { n /= 10 ; i += 1 ; } i }
};
}
