// Generated macro for ConstVariableValue (enum)
macro_rules! Depcrate_infer_unify_keyConstVariableValue {
() => {
// Module: crate::infer::unify_key
// Provides: {"ConstVariableValue"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] pub (crate) enum ConstVariableValue < 'tcx > { Known { value : ty :: Const < 'tcx > } , Unknown { origin : ConstVariableOrigin , universe : ty :: UniverseIndex } , }
};
}
