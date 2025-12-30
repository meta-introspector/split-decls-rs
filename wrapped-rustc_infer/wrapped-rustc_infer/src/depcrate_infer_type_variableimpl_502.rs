// Generated macro for impl_502 (impl)
macro_rules! Depcrate_infer_type_variableimpl_502 {
() => {
// Module: crate::infer::type_variable
// Provides: {"impl_502"}
// Dependencies: {}
# [doc = " Convert from a specific kind of undo to the more general UndoLog"] impl < 'tcx > From < sv :: UndoLog < ut :: Delegate < TyVidEqKey < 'tcx > > > > for UndoLog < 'tcx > { fn from (l : sv :: UndoLog < ut :: Delegate < TyVidEqKey < 'tcx > > >) -> Self { UndoLog :: EqRelation (l) } }
};
}
