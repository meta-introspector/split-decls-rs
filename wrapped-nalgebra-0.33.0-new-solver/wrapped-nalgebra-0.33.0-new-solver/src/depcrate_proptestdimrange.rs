// Generated macro for DimRange (struct)
macro_rules! Depcrate_proptestDimRange {
() => {
// Module: crate::proptest
// Provides: {"DimRange"}
// Dependencies: {}
# [doc = " A range of allowed dimensions for use in generation of matrices."] # [doc = ""] # [doc = " The `DimRange` type is used to encode the range of dimensions that can be used for generation"] # [doc = " of matrices with `proptest`. In most cases, you do not need to concern yourself with"] # [doc = " `DimRange` directly, as it supports conversion from other types such as `U3` or inclusive"] # [doc = " ranges such as `5 ..= 6`. The latter example corresponds to dimensions from (inclusive)"] # [doc = " `Dyn(5)` to `Dyn(6)` (inclusive)."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct DimRange < D = Dyn > (RangeInclusive < D >) ;
};
}
