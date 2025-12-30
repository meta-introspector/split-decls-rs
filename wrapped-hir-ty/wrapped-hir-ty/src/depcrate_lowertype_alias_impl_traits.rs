// Generated macro for type_alias_impl_traits (function)
macro_rules! Depcrate_lowertype_alias_impl_traits {
() => {
// Module: crate::lower
// Provides: {"type_alias_impl_traits"}
// Dependencies: {}
pub (crate) fn type_alias_impl_traits (db : & dyn HirDatabase , def : hir_def :: TypeAliasId ,) -> Option < Arc < Binders < ImplTraits > > > { let data = db . type_alias_signature (def) ; let resolver = def . resolver (db) ; let mut ctx = TyLoweringContext :: new (db , & resolver , & data . store , def . into () , LifetimeElisionKind :: AnonymousReportError ,) . with_impl_trait_mode (ImplTraitLoweringMode :: Opaque) . with_type_param_mode (ParamLoweringMode :: Variable) ; if let Some (type_ref) = data . ty { let _ty = ctx . lower_ty (type_ref) ; } let type_alias_impl_traits = ImplTraits { impl_traits : ctx . impl_trait_mode . opaque_type_data } ; if type_alias_impl_traits . impl_traits . is_empty () { None } else { let generics = generics (db , def . into ()) ; Some (Arc :: new (make_binders (db , & generics , type_alias_impl_traits))) } }
};
}
