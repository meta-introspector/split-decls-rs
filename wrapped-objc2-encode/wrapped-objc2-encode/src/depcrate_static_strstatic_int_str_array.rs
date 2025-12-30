// Generated macro for static_int_str_array (function)
macro_rules! Depcrate_static_strstatic_int_str_array {
() => {
// Module: crate::static_str
// Provides: {"static_int_str_array"}
// Dependencies: {}
pub (crate) const fn static_int_str_array < const RES : usize > (mut n : u64) -> [u8 ; RES] { let mut res : [u8 ; RES] = [0 ; RES] ; let mut i = 0 ; if n == 0 { res [0] = b'0' ; return res ; } while n > 0 { res [i] = b'0' + (n % 10) as u8 ; n /= 10 ; i += 1 ; } let mut rev : [u8 ; RES] = [0 ; RES] ; let mut rev_i = 0 ; while 0 < i { i -= 1 ; rev [rev_i] = res [i] ; n /= 10 ; rev_i += 1 ; } rev }
};
}
