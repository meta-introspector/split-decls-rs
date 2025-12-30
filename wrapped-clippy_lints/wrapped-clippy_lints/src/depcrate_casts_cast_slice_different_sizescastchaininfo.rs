// Generated macro for CastChainInfo (struct)
macro_rules! Depcrate_casts_cast_slice_different_sizesCastChainInfo {
() => {
// Module: crate::casts::cast_slice_different_sizes
// Provides: {"CastChainInfo"}
// Dependencies: {}
struct CastChainInfo < 'tcx > { # [doc = " The left most part of the cast chain, or in other words, the first cast in the chain"] # [doc = " Used for diagnostics"] left_cast : & 'tcx Expr < 'tcx > , # [doc = " The starting type of the cast chain"] start_ty : TypeAndMut < 'tcx > , # [doc = " The final type of the cast chain"] end_ty : TypeAndMut < 'tcx > , }
};
}
