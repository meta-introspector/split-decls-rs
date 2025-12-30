// Generated macro for RegionVariableValue (enum)
macro_rules! Depcrate_infer_unify_keyRegionVariableValue {
() => {
// Module: crate::infer::unify_key
// Provides: {"RegionVariableValue"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] pub (crate) enum RegionVariableValue < 'tcx > { Known { value : ty :: Region < 'tcx > } , Unknown { universe : ty :: UniverseIndex } , }
};
}
