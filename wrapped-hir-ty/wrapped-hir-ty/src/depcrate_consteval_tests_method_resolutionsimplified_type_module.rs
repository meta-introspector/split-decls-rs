// Generated macro for simplified_type_module (function)
macro_rules! Depcrate_consteval_tests_method_resolutionsimplified_type_module {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"simplified_type_module"}
// Dependencies: {}
pub fn simplified_type_module (db : & dyn HirDatabase , ty : & SimplifiedType) -> Option < ModuleId > { match ty . def () ? { SolverDefId :: AdtId (id) => Some (id . module (db)) , SolverDefId :: TypeAliasId (id) => Some (id . module (db)) , SolverDefId :: TraitId (id) => Some (id . module (db)) , _ => None , } }
};
}
