// Generated macro for DimName (trait)
macro_rules! Depcrate_base_dimensionDimName {
() => {
// Module: crate::base::dimension
// Provides: {"DimName"}
// Dependencies: {}
# [doc = " Trait implemented exclusively by type-level integers."] pub trait DimName : Dim { const USIZE : usize ; # [doc = " The name of this dimension, i.e., the singleton `Self`."] fn name () -> Self ; # [doc = " The value of this dimension."] fn dim () -> usize ; }
};
}
