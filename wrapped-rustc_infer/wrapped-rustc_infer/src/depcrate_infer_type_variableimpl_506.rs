// Generated macro for impl_506 (impl)
macro_rules! Depcrate_infer_type_variableimpl_506 {
() => {
// Module: crate::infer::type_variable
// Provides: {"impl_506"}
// Dependencies: {}
impl < 'tcx > Rollback < UndoLog < 'tcx > > for TypeVariableStorage < 'tcx > { fn reverse (& mut self , undo : UndoLog < 'tcx >) { match undo { UndoLog :: EqRelation (undo) => self . eq_relations . reverse (undo) , UndoLog :: SubRelation (undo) => self . sub_unification_table . reverse (undo) , } } }
};
}
