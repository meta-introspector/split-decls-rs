// Generated macro for ConstMultiDistribution (trait)
macro_rules! Depcrate_multiConstMultiDistribution {
() => {
// Module: crate::multi
// Provides: {"ConstMultiDistribution"}
// Dependencies: {}
# [doc = " An extension of [`MultiDistribution`] for multi-dimensional distributions of fixed dimension"] # [doc = ""] # [doc = " Implementations may also implement `Distribution<[T; SAMPLE_LEN]>`."] pub trait ConstMultiDistribution < T > : MultiDistribution < T > { # [doc = " Constant sample length (dimension of the distribution)"] const SAMPLE_LEN : usize ; }
};
}
