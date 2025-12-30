// Generated macro for impl_504 (impl)
macro_rules! Depcrate_infer_type_variableimpl_504 {
() => {
// Module: crate::infer::type_variable
// Provides: {"impl_504"}
// Dependencies: {}
impl < 'tcx > Rollback < sv :: UndoLog < ut :: Delegate < TyVidEqKey < 'tcx > > > > for TypeVariableStorage < 'tcx > { fn reverse (& mut self , undo : sv :: UndoLog < ut :: Delegate < TyVidEqKey < 'tcx > > >) { self . eq_relations . reverse (undo) } }
};
}
