// Generated macro for UndoLog (enum)
macro_rules! Depcrate_infer_region_constraintsUndoLog {
() => {
// Module: crate::infer::region_constraints
// Provides: {"UndoLog"}
// Dependencies: {}
# [derive (Copy , Clone , PartialEq)] pub (crate) enum UndoLog < 'tcx > { # [doc = " We added `RegionVid`."] AddVar (RegionVid) , # [doc = " We added the given `constraint`."] AddConstraint (usize) , # [doc = " We added the given `verify`."] AddVerify (usize) , # [doc = " We added a GLB/LUB \"combination variable\"."] AddCombination (CombineMapType , TwoRegions < 'tcx >) , }
};
}
