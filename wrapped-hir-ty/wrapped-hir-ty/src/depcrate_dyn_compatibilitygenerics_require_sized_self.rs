// Generated macro for generics_require_sized_self (function)
macro_rules! Depcrate_dyn_compatibilitygenerics_require_sized_self {
() => {
// Module: crate::dyn_compatibility
// Provides: {"generics_require_sized_self"}
// Dependencies: {}
pub fn generics_require_sized_self (db : & dyn HirDatabase , def : GenericDefId) -> bool { let krate = def . module (db) . krate () ; let Some (sized) = LangItem :: Sized . resolve_trait (db , krate) else { return false ; } ; let interner = DbInterner :: new_with (db , Some (krate) , None) ; let predicates = GenericPredicates :: query_explicit (db , def) ; elaborate :: elaborate (interner , predicates . iter_identity_copied ()) . any (| pred | { match pred . kind () . skip_binder () { ClauseKind :: Trait (trait_pred) => { if sized == trait_pred . def_id () . 0 && let rustc_type_ir :: TyKind :: Param (param_ty) = trait_pred . trait_ref . self_ty () . kind () && param_ty . index == 0 { true } else { false } } _ => false , } }) }
};
}
