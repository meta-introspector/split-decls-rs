// Generated macro for RegionBoundPairs (type)
macro_rules! Depcrate_infer_outlives_envRegionBoundPairs {
() => {
// Module: crate::infer::outlives::env
// Provides: {"RegionBoundPairs"}
// Dependencies: {}
# [doc = " \"Region-bound pairs\" tracks outlives relations that are known to"] # [doc = " be true, either because of explicit where-clauses like `T: 'a` or"] # [doc = " because of implied bounds."] pub type RegionBoundPairs < 'tcx > = FxIndexSet < ty :: OutlivesPredicate < 'tcx , GenericKind < 'tcx > > > ;
};
}
