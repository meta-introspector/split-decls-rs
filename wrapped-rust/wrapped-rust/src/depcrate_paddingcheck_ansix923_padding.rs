// Generated macro for check_ansix923_padding (function)
macro_rules! Depcrate_paddingcheck_ansix923_padding {
() => {
// Module: crate::padding
// Provides: {"check_ansix923_padding"}
// Dependencies: {}
fn check_ansix923_padding (data : & [u8]) -> bool { let mut mismatch = 0 ; let pad_size = * data . last () . unwrap () ; let len : u8 = data . len () . try_into () . expect ("data too long") ; for (i , b) in (1 .. len) . zip (data [.. data . len () - 1] . iter () . rev ()) { let mask = constant_time_lt (i , pad_size) ; mismatch |= mask & b ; } mismatch |= ! constant_time_lt (0 , pad_size) ; mismatch |= constant_time_lt (len , pad_size) ; mismatch |= mismatch >> 4 ; mismatch |= mismatch >> 2 ; mismatch |= mismatch >> 1 ; (mismatch & 1) == 0 }
};
}
