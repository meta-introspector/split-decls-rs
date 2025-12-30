// Generated macro for sc_mul (function)
macro_rules! Depcrate_edwards25519sc_mul {
() => {
// Module: crate::edwards25519
// Provides: {"sc_mul"}
// Dependencies: {}
# [cfg (feature = "blind-keys")] pub fn sc_mul (a : & [u8] , b : & [u8]) -> [u8 ; 32] { let mut s = [0u8 ; 32] ; sc_muladd (& mut s , a , b , & [0 ; 32]) ; s }
};
}
