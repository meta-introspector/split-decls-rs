// Generated macro for impl_trait_with_diagnostics_query (function)
macro_rules! Depcrate_lowerimpl_trait_with_diagnostics_query {
() => {
// Module: crate::lower
// Provides: {"impl_trait_with_diagnostics_query"}
// Dependencies: {}
pub (crate) fn impl_trait_with_diagnostics_query < 'db > (db : & 'db dyn HirDatabase , impl_id : ImplId ,) -> Option < (EarlyBinder < 'db , TraitRef < 'db > > , Diagnostics) > { let impl_data = db . impl_signature (impl_id) ; let resolver = impl_id . resolver (db) ; let mut ctx = TyLoweringContext :: new (db , & resolver , & impl_data . store , impl_id . into () , LifetimeElisionKind :: AnonymousCreateParameter { report_in_path : true } ,) ; let self_ty = db . impl_self_ty (impl_id) . skip_binder () ; let target_trait = impl_data . target_trait . as_ref () ? ; let trait_ref = EarlyBinder :: bind (ctx . lower_trait_ref (target_trait , self_ty) ?) ; Some ((trait_ref , create_diagnostics (ctx . diagnostics))) }
};
}
