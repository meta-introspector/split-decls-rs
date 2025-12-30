// Generated macro for ReduceNonZero (trait)
macro_rules! Depcrate_opsReduceNonZero {
() => {
// Module: crate::ops
// Provides: {"ReduceNonZero"}
// Dependencies: {}
# [doc = " Modular reduction to a non-zero output."] # [doc = ""] # [doc = " This trait is primarily intended for use by curve implementations such"] # [doc = " as the `k256` and `p256` crates."] # [doc = ""] # [doc = " End users should use the [`Reduce`] impl on"] # [doc = " [`NonZeroScalar`][`crate::NonZeroScalar`] instead."] pub trait ReduceNonZero < T > : Reduce < T > { # [doc = " Perform a modular reduction, returning a field element."] fn reduce_nonzero (n : & T) -> Self ; }
};
}
