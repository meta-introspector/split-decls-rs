// Generated macro for p (function)
macro_rules! Depcrate_compress_shortp {
() => {
// Module: crate::compress_short
// Provides: {"p"}
// Dependencies: {}
pub (crate) fn p (h : & [u64 ; COLS]) -> [u64 ; COLS] { let mut p = * h ; for i in 0 .. ROUNDS { p = rndp (p , i << 56) ; } for i in 0 .. COLS { p [i] ^= h [i] ; } p }
};
}
