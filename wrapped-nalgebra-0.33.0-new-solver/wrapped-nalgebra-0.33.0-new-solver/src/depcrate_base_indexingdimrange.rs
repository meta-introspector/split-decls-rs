// Generated macro for DimRange (trait)
macro_rules! Depcrate_base_indexingDimRange {
() => {
// Module: crate::base::indexing
// Provides: {"DimRange"}
// Dependencies: {}
trait DimRange < D : Dim > { # [doc = " The number of elements indexed by this range."] type Length : Dim ; # [doc = " The lower bound of the range, inclusive."] fn lower (& self , dimension : D) -> usize ; # [doc = " The number of elements included in the range."] fn length (& self , dimension : D) -> Self :: Length ; # [doc = " Produces true if `Self` is contained within `dimension`."] fn contained_by (& self , dimension : D) -> bool ; }
};
}
