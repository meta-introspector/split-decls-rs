// Generated macro for SymmetricDifference (struct)
macro_rules! DepcrateSymmetricDifference {
() => {
// Module: crate
// Provides: {"SymmetricDifference"}
// Dependencies: {}
# [doc = " An iterator producing elements in the symmetric difference of two sets."] # [doc = ""] # [doc = " This struct is created by the [`FixedBitSet::symmetric_difference`] method."] pub struct SymmetricDifference < 'a > { iter : Chain < Difference < 'a > , Difference < 'a > > , }
};
}
