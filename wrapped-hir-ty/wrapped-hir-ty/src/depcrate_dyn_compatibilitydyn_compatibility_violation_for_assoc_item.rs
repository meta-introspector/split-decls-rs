// Generated macro for dyn_compatibility_violation_for_assoc_item (function)
macro_rules! Depcrate_dyn_compatibilitydyn_compatibility_violation_for_assoc_item {
() => {
// Module: crate::dyn_compatibility
// Provides: {"dyn_compatibility_violation_for_assoc_item"}
// Dependencies: {}
fn dyn_compatibility_violation_for_assoc_item < F > (db : & dyn HirDatabase , trait_ : TraitId , item : AssocItemId , cb : & mut F ,) -> ControlFlow < () > where F : FnMut (DynCompatibilityViolation) -> ControlFlow < () > , { if generics_require_sized_self (db , item . into ()) { return ControlFlow :: Continue (()) ; } match item { AssocItemId :: ConstId (it) => cb (DynCompatibilityViolation :: AssocConst (it)) , AssocItemId :: FunctionId (it) => { virtual_call_violations_for_method (db , trait_ , it , & mut | mvc | { cb (DynCompatibilityViolation :: Method (it , mvc)) }) } AssocItemId :: TypeAliasId (it) => { let def_map = CrateRootModuleId :: from (trait_ . krate (db)) . def_map (db) ; if def_map . is_unstable_feature_enabled (& intern :: sym :: generic_associated_type_extended) { ControlFlow :: Continue (()) } else { let generic_params = db . generic_params (item . into ()) ; if ! generic_params . is_empty () { cb (DynCompatibilityViolation :: GAT (it)) } else { ControlFlow :: Continue (()) } } } } }
};
}
