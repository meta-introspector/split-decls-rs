// Generated macro for impl_512 (impl)
macro_rules! Depcrate_infer_type_variableimpl_512 {
() => {
// Module: crate::infer::type_variable
// Provides: {"impl_512"}
// Dependencies: {}
impl < 'tcx > TypeVariableValue < 'tcx > { # [doc = " If this value is known, returns the type it is known to be."] # [doc = " Otherwise, `None`."] pub (crate) fn known (& self) -> Option < Ty < 'tcx > > { match * self { TypeVariableValue :: Unknown { .. } => None , TypeVariableValue :: Known { value } => Some (value) , } } pub (crate) fn is_unknown (& self) -> bool { match * self { TypeVariableValue :: Unknown { .. } => true , TypeVariableValue :: Known { .. } => false , } } }
};
}
