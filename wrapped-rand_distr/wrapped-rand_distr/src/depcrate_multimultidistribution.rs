// Generated macro for MultiDistribution (trait)
macro_rules! Depcrate_multiMultiDistribution {
() => {
// Module: crate::multi
// Provides: {"MultiDistribution"}
// Dependencies: {}
# [doc = " A standard abstraction for distributions with multi-dimensional results"] # [doc = ""] # [doc = " Implementations may also implement `Distribution<Vec<T>>`."] pub trait MultiDistribution < T > { # [doc = " The length of a sample (dimension of the distribution)"] fn sample_len (& self) -> usize ; # [doc = " Sample a multi-dimensional result from the distribution"] # [doc = ""] # [doc = " The result is written to `output`. Implementations should assert that"] # [doc = " `output.len()` equals the result of [`Self::sample_len`]."] fn sample_to_slice < R : Rng + ? Sized > (& self , rng : & mut R , output : & mut [T]) ; }
};
}
