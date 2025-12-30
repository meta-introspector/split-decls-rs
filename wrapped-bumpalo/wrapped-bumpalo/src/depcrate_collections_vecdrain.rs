// Generated macro for Drain (struct)
macro_rules! Depcrate_collections_vecDrain {
() => {
// Module: crate::collections::vec
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator for `Vec<'bump, T>`."] # [doc = ""] # [doc = " This `struct` is created by the [`Vec::drain`] method."] pub struct Drain < 'a , 'bump , T : 'a + 'bump > { # [doc = " Index of tail to preserve"] tail_start : usize , # [doc = " Length of tail"] tail_len : usize , # [doc = " Current remaining range to remove"] iter : slice :: Iter < 'a , T > , vec : NonNull < Vec < 'bump , T > > , }
};
}
