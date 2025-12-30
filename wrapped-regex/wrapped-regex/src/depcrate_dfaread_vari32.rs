// Generated macro for read_vari32 (function)
macro_rules! Depcrate_dfaread_vari32 {
() => {
// Module: crate::dfa
// Provides: {"read_vari32"}
// Dependencies: {}
# [doc = " https://developers.google.com/protocol-buffers/docs/encoding#varints"] fn read_vari32 (data : & [u8]) -> (i32 , usize) { let (un , i) = read_varu32 (data) ; let mut n = (un >> 1) as i32 ; if un & 1 != 0 { n = ! n ; } (n , i) }
};
}
