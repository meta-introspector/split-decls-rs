// Generated macro for impl_537 (impl)
macro_rules! Depcrate_infer_unify_keyimpl_537 {
() => {
// Module: crate::infer::unify_key
// Provides: {"impl_537"}
// Dependencies: {}
impl < 'tcx > ConstVariableValue < 'tcx > { # [doc = " If this value is known, returns the const it is known to be."] # [doc = " Otherwise, `None`."] pub (crate) fn known (& self) -> Option < ty :: Const < 'tcx > > { match * self { ConstVariableValue :: Unknown { .. } => None , ConstVariableValue :: Known { value } => Some (value) , } } }
};
}
