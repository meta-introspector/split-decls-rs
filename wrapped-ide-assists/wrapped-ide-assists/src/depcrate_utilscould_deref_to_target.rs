// Generated macro for could_deref_to_target (function)
macro_rules! Depcrate_utilscould_deref_to_target {
() => {
// Module: crate::utils
// Provides: {"could_deref_to_target"}
// Dependencies: {}
fn could_deref_to_target (ty : & hir :: Type < '_ > , target : & hir :: Type < '_ > , db : & dyn HirDatabase) -> bool { let ty_ref = ty . add_reference (hir :: Mutability :: Shared) ; let target_ref = target . add_reference (hir :: Mutability :: Shared) ; ty_ref . could_coerce_to (db , & target_ref) }
};
}
