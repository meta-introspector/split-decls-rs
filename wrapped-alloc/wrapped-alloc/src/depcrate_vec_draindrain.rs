// Generated macro for Drain (struct)
macro_rules! Depcrate_vec_drainDrain {
() => {
// Module: crate::vec::drain
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator for `Vec<T>`."] # [doc = ""] # [doc = " This `struct` is created by [`Vec::drain`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let mut v = vec![0, 1, 2];"] # [doc = " let iter: std::vec::Drain<'_, _> = v.drain(..);"] # [doc = " ```"] # [stable (feature = "drain" , since = "1.6.0")] pub struct Drain < 'a , T : 'a , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator + 'a = Global , > { # [doc = " Index of tail to preserve"] pub (super) tail_start : usize , # [doc = " Length of tail"] pub (super) tail_len : usize , # [doc = " Current remaining range to remove"] pub (super) iter : slice :: Iter < 'a , T > , pub (super) vec : NonNull < Vec < T , A > > , }
};
}
