// Generated macro for ss (function)
macro_rules! Depcrate_compressorss {
() => {
// Module: crate::compressor
// Provides: {"ss"}
// Dependencies: {}
# [doc = " two Sboxes computed in parallel; each Sbox implements S0 and S1, selected by a constant bit"] # [inline (always)] fn ss < M : Machine > (state : X8 < M > , mut k : M :: u128x2) -> X8 < M > { let mut m = state . zip () ; m . 3 = ! m . 3 ; m . 0 ^= m . 2 . andnot (k) ; k ^= m . 0 & m . 1 ; m . 0 ^= m . 3 & m . 2 ; m . 3 ^= m . 1 . andnot (m . 2) ; m . 1 ^= m . 0 & m . 2 ; m . 2 ^= m . 3 . andnot (m . 0) ; m . 0 ^= m . 1 | m . 3 ; m . 3 ^= m . 1 & m . 2 ; m . 2 ^= k ; m . 1 ^= k & m . 0 ; X8 :: unzip (m) }
};
}
