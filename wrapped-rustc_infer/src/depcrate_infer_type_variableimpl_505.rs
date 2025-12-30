// Generated macro for impl_505 (impl)
macro_rules! Depcrate_infer_type_variableimpl_505 {
() => {
// Module: crate::infer::type_variable
// Provides: {"impl_505"}
// Dependencies: {}
impl < 'tcx > Rollback < sv :: UndoLog < ut :: Delegate < TyVidSubKey > > > for TypeVariableStorage < 'tcx > { fn reverse (& mut self , undo : sv :: UndoLog < ut :: Delegate < TyVidSubKey > >) { self . sub_unification_table . reverse (undo) } }
};
}
