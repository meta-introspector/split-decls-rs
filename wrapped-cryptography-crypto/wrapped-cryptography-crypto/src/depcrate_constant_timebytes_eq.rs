// Generated macro for bytes_eq (function)
macro_rules! Depcrate_constant_timebytes_eq {
() => {
// Module: crate::constant_time
// Provides: {"bytes_eq"}
// Dependencies: {}
# [doc = " Performs a constant-time comparison of two byte slices."] pub fn bytes_eq (a : & [u8] , b : & [u8]) -> bool { if a . len () != b . len () { return false ; } openssl :: memcmp :: eq (a , b) }
};
}
