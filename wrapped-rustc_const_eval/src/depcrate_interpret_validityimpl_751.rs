// Generated macro for impl_751 (impl)
macro_rules! Depcrate_interpret_validityimpl_751 {
() => {
// Module: crate::interpret::validity
// Provides: {"impl_751"}
// Dependencies: {}
impl CtfeValidationMode { fn allow_immutable_unsafe_cell (self) -> bool { match self { CtfeValidationMode :: Static { .. } => false , CtfeValidationMode :: Promoted { .. } => false , CtfeValidationMode :: Const { allow_immutable_unsafe_cell , .. } => { allow_immutable_unsafe_cell } } } }
};
}
