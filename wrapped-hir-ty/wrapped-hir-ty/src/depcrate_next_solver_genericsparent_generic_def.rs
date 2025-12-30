// Generated macro for parent_generic_def (function)
macro_rules! Depcrate_next_solver_genericsparent_generic_def {
() => {
// Module: crate::next_solver::generics
// Provides: {"parent_generic_def"}
// Dependencies: {}
pub (crate) fn parent_generic_def (db : & dyn DefDatabase , def : GenericDefId) -> Option < GenericDefId > { let container = match def { GenericDefId :: FunctionId (it) => it . lookup (db) . container , GenericDefId :: TypeAliasId (it) => it . lookup (db) . container , GenericDefId :: ConstId (it) => it . lookup (db) . container , GenericDefId :: StaticId (_) | GenericDefId :: AdtId (_) | GenericDefId :: TraitId (_) | GenericDefId :: ImplId (_) => return None , } ; match container { ItemContainerId :: ImplId (it) => Some (it . into ()) , ItemContainerId :: TraitId (it) => Some (it . into ()) , ItemContainerId :: ModuleId (_) | ItemContainerId :: ExternBlockId (_) => None , } }
};
}
