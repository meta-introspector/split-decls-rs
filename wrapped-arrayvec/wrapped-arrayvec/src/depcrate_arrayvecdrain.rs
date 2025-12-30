// Generated macro for Drain (struct)
macro_rules! Depcrate_arrayvecDrain {
() => {
// Module: crate::arrayvec
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator for `ArrayVec`."] pub struct Drain < 'a , T : 'a , const CAP : usize > { # [doc = " Index of tail to preserve"] tail_start : usize , # [doc = " Length of tail"] tail_len : usize , # [doc = " Current remaining range to remove"] iter : slice :: Iter < 'a , T > , vec : * mut ArrayVec < T , CAP > , }
};
}
