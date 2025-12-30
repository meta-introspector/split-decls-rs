// Generated macro for column (function)
macro_rules! Depcrate_compress_longcolumn {
() => {
// Module: crate::compress_long
// Provides: {"column"}
// Dependencies: {}
# [inline (always)] fn column (x : & [u64 ; COLS] , c : [usize ; 8]) -> u64 { let mut t = 0 ; for i in 0 .. 8 { let sl = 8 * (7 - i) ; let idx = ((x [c [i]] >> sl) & 0xFF) as usize ; t ^= TABLE [i] [idx] ; } t }
};
}
