// Generated macro for set_bit (function)
macro_rules! Depcrate_x509_certificateset_bit {
() => {
// Module: crate::x509::certificate
// Provides: {"set_bit"}
// Dependencies: {}
pub (crate) fn set_bit (vals : & mut [u8] , n : usize , set : bool) { let idx = n / 8 ; let v = 1 << (7 - (n & 0x07)) ; if set { vals [idx] |= v ; } }
};
}
