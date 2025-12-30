// Generated macro for FreeRegionMap (struct)
macro_rules! Depcrate_infer_free_regionsFreeRegionMap {
() => {
// Module: crate::infer::free_regions
// Provides: {"FreeRegionMap"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct FreeRegionMap < 'tcx > { # [doc = " Stores the relation `a < b`, where `a` and `b` are regions."] # [doc = ""] # [doc = " Invariant: only free regions like `'x` or `'static` are stored"] # [doc = " in this relation, not scopes."] pub (crate) relation : TransitiveRelation < Region < 'tcx > > , }
};
}
