// Generated macro for seq_matmulz (function)
macro_rules! Depcrate_matmulseq_matmulz {
() => {
// Module: crate::matmul
// Provides: {"seq_matmulz"}
// Dependencies: {}
# [inline (never)] pub fn seq_matmulz (a : & [f32] , b : & [f32] , dest : & mut [f32]) { assert ! (a . len () == b . len () && a . len () == dest . len ()) ; assert ! (a . len () . count_ones () == 1 && a . len () . trailing_zeros () . is_multiple_of (2)) ; for d in dest . iter_mut () { * d = 0.0 ; } let n = dest . len () ; for (ij , d) in dest . iter_mut () . enumerate () { let i = ij & 0xaaaa_aaaa ; let j = ij & 0x5555_5555 ; let mut sum = 0.0 ; for k in SplayedBitsCounter :: new (n) { sum += unsafe { a . get_unchecked (i | k) * b . get_unchecked ((k << 1) | j) } ; } * d = sum ; } }
};
}
