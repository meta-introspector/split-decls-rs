// Generated macro for BatchNormalize (trait)
macro_rules! Depcrate_pointBatchNormalize {
() => {
// Module: crate::point
// Provides: {"BatchNormalize"}
// Dependencies: {}
# [doc = " Normalize point(s) in projective representation by converting them to their affine ones."] # [cfg (feature = "arithmetic")] pub trait BatchNormalize < Points : ? Sized > { # [doc = " The output of the batch normalization; a container of affine points."] type Output ; # [doc = " Perform a batched conversion to affine representation on a sequence of projective points"] # [doc = " at an amortized cost that should be practically as efficient as a single conversion."] # [doc = " Internally, implementors should rely upon `InvertBatch`."] fn batch_normalize (points : & Points) -> < Self as BatchNormalize < Points > > :: Output ; }
};
}
