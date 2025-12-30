// Generated macro for check_pkcs7_padding (function)
macro_rules! Depcrate_paddingcheck_pkcs7_padding {
() => {
// Module: crate::padding
// Provides: {"check_pkcs7_padding"}
// Dependencies: {}
fn check_pkcs7_padding (data : & [u8]) -> bool { let mut mismatch = 0 ; let pad_size = * data . last () . unwrap () ; let len : u8 = data . len () . try_into () . expect ("data too long") ; for (i , b) in (0 .. len) . zip (data . iter () . rev ()) { let mask = constant_time_lt (i , pad_size) ; mismatch |= mask & (pad_size ^ b) ; } mismatch |= ! constant_time_lt (0 , pad_size) ; mismatch |= constant_time_lt (len , pad_size) ; mismatch |= mismatch >> 4 ; mismatch |= mismatch >> 2 ; mismatch |= mismatch >> 1 ; (mismatch & 1) == 0 }
};
}
