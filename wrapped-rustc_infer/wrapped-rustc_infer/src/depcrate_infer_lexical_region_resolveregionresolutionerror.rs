// Generated macro for RegionResolutionError (enum)
macro_rules! Depcrate_infer_lexical_region_resolveRegionResolutionError {
() => {
// Module: crate::infer::lexical_region_resolve
// Provides: {"RegionResolutionError"}
// Dependencies: {}
# [derive (Clone , Debug)] pub enum RegionResolutionError < 'tcx > { # [doc = " `ConcreteFailure(o, a, b)`:"] # [doc = ""] # [doc = " `o` requires that `a <= b`, but this does not hold"] ConcreteFailure (SubregionOrigin < 'tcx > , Region < 'tcx > , Region < 'tcx >) , # [doc = " `GenericBoundFailure(p, s, a)`:"] # [doc = ""] # [doc = " The parameter/associated-type `p` must be known to outlive the lifetime"] # [doc = " `a` (but none of the known bounds are sufficient)."] GenericBoundFailure (SubregionOrigin < 'tcx > , GenericKind < 'tcx > , Region < 'tcx >) , # [doc = " `SubSupConflict(v, v_origin, sub_origin, sub_r, sup_origin, sup_r)`:"] # [doc = ""] # [doc = " Could not infer a value for `v` (which has origin `v_origin`)"] # [doc = " because `sub_r <= v` (due to `sub_origin`) but `v <= sup_r` (due to `sup_origin`) and"] # [doc = " `sub_r <= sup_r` does not hold."] SubSupConflict (RegionVid , RegionVariableOrigin , SubregionOrigin < 'tcx > , Region < 'tcx > , SubregionOrigin < 'tcx > , Region < 'tcx > , Vec < Span > ,) , # [doc = " Indicates a `'b: 'a` constraint where `'a` is in a universe that"] # [doc = " cannot name the placeholder `'b`."] UpperBoundUniverseConflict (RegionVid , RegionVariableOrigin , ty :: UniverseIndex , SubregionOrigin < 'tcx > , Region < 'tcx > ,) , CannotNormalize (ty :: PolyTypeOutlivesPredicate < 'tcx > , SubregionOrigin < 'tcx >) , }
};
}
