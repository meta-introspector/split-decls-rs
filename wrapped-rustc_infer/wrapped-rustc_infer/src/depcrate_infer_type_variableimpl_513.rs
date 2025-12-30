// Generated macro for impl_513 (impl)
macro_rules! Depcrate_infer_type_variableimpl_513 {
() => {
// Module: crate::infer::type_variable
// Provides: {"impl_513"}
// Dependencies: {}
impl < 'tcx > TypeVariableStorage < 'tcx > { # [inline] pub (crate) fn with_log < 'a > (& 'a mut self , undo_log : & 'a mut InferCtxtUndoLogs < 'tcx > ,) -> TypeVariableTable < 'a , 'tcx > { TypeVariableTable { storage : self , undo_log } } # [inline] pub (crate) fn eq_relations_ref (& self) -> & ut :: UnificationTableStorage < TyVidEqKey < 'tcx > > { & self . eq_relations } pub (super) fn finalize_rollback (& mut self) { debug_assert ! (self . values . len () >= self . eq_relations . len ()) ; self . values . truncate (self . eq_relations . len ()) ; } }
};
}
