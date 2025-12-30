// Generated macro for return_type_impl_traits (function)
macro_rules! Depcrate_lowerreturn_type_impl_traits {
() => {
// Module: crate::lower
// Provides: {"return_type_impl_traits"}
// Dependencies: {}
pub (crate) fn return_type_impl_traits (db : & dyn HirDatabase , def : hir_def :: FunctionId ,) -> Option < Arc < Binders < ImplTraits > > > { let data = db . function_signature (def) ; let resolver = def . resolver (db) ; let mut ctx_ret = TyLoweringContext :: new (db , & resolver , & data . store , def . into () , LifetimeElisionKind :: Infer) . with_impl_trait_mode (ImplTraitLoweringMode :: Opaque) . with_type_param_mode (ParamLoweringMode :: Variable) ; if let Some (ret_type) = data . ret_type { let _ret = ctx_ret . lower_ty (ret_type) ; } let generics = generics (db , def . into ()) ; let return_type_impl_traits = ImplTraits { impl_traits : ctx_ret . impl_trait_mode . opaque_type_data } ; if return_type_impl_traits . impl_traits . is_empty () { None } else { Some (Arc :: new (make_binders (db , & generics , return_type_impl_traits))) } }
};
}
