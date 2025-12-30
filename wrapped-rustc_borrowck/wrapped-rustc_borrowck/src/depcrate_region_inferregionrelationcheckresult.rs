// Generated macro for RegionRelationCheckResult (enum)
macro_rules! Depcrate_region_inferRegionRelationCheckResult {
() => {
// Module: crate::region_infer
// Provides: {"RegionRelationCheckResult"}
// Dependencies: {}
# [doc = " When we have an unmet lifetime constraint, we try to propagate it outward (e.g. to a closure"] # [doc = " environment). If we can't, it is an error."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] enum RegionRelationCheckResult { Ok , Propagated , Error , }
};
}
