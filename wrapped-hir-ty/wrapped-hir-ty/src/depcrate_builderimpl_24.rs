// Generated macro for impl_24 (impl)
macro_rules! Depcrate_builderimpl_24 {
() => {
// Module: crate::builder
// Provides: {"impl_24"}
// Dependencies: {}
impl TyBuilder < Binders < Ty > > { pub fn def_ty (db : & dyn HirDatabase , def : TyDefId , parent_subst : Option < Substitution > ,) -> TyBuilder < Binders < Ty > > { let poly_ty = db . ty (def) ; let id : GenericDefId = match def { TyDefId :: BuiltinType (_) => { assert ! (parent_subst . is_none ()) ; return TyBuilder :: new_empty (poly_ty) ; } TyDefId :: AdtId (id) => id . into () , TyDefId :: TypeAliasId (id) => id . into () , } ; TyBuilder :: subst_for_def (db , id , parent_subst) . with_data (poly_ty) } pub fn impl_self_ty (db : & dyn HirDatabase , def : hir_def :: ImplId) -> TyBuilder < Binders < Ty > > { TyBuilder :: subst_for_def (db , def , None) . with_data (db . impl_self_ty (def)) } }
};
}
