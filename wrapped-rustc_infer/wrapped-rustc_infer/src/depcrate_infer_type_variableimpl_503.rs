// Generated macro for impl_503 (impl)
macro_rules! Depcrate_infer_type_variableimpl_503 {
() => {
// Module: crate::infer::type_variable
// Provides: {"impl_503"}
// Dependencies: {}
# [doc = " Convert from a specific kind of undo to the more general UndoLog"] impl < 'tcx > From < sv :: UndoLog < ut :: Delegate < TyVidSubKey > > > for UndoLog < 'tcx > { fn from (l : sv :: UndoLog < ut :: Delegate < TyVidSubKey > >) -> Self { UndoLog :: SubRelation (l) } }
};
}
