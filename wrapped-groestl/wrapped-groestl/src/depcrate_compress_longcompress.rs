// Generated macro for compress (function)
macro_rules! Depcrate_compress_longcompress {
() => {
// Module: crate::compress_long
// Provides: {"compress"}
// Dependencies: {}
pub (crate) fn compress (h : & mut [u64 ; COLS] , block : & [u8 ; 128]) { let mut q = [0u64 ; COLS] ; for (chunk , v) in block . chunks_exact (8) . zip (q . iter_mut ()) { * v = u64 :: from_be_bytes (chunk . try_into () . unwrap ()) ; } let mut p = [0u64 ; COLS] ; for i in 0 .. COLS { p [i] = h [i] ^ q [i] ; } for i in 0 .. ROUNDS { q = rndq (q , i) ; } for i in 0 .. ROUNDS { p = rndp (p , i << 56) ; } for i in 0 .. COLS { h [i] ^= q [i] ^ p [i] ; } }
};
}
