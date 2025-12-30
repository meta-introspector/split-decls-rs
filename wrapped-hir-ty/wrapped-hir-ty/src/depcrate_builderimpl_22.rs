// Generated macro for impl_22 (impl)
macro_rules! Depcrate_builderimpl_22 {
() => {
// Module: crate::builder
// Provides: {"impl_22"}
// Dependencies: {}
impl TyBuilder < TypeAliasId > { pub fn assoc_type_projection (db : & dyn HirDatabase , def : TypeAliasId , parent_subst : Option < Substitution > ,) -> TyBuilder < TypeAliasId > { TyBuilder :: subst_for_def (db , def , parent_subst) . with_data (def) } pub fn build (self) -> ProjectionTy { let (type_alias , substitution) = self . build_internal () ; ProjectionTy { associated_ty_id : to_assoc_type_id (type_alias) , substitution } } }
};
}
