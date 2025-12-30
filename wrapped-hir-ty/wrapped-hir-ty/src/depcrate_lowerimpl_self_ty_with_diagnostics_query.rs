// Generated macro for impl_self_ty_with_diagnostics_query (function)
macro_rules! Depcrate_lowerimpl_self_ty_with_diagnostics_query {
() => {
// Module: crate::lower
// Provides: {"impl_self_ty_with_diagnostics_query"}
// Dependencies: {}
pub (crate) fn impl_self_ty_with_diagnostics_query < 'db > (db : & 'db dyn HirDatabase , impl_id : ImplId ,) -> (EarlyBinder < 'db , Ty < 'db > > , Diagnostics) { let resolver = impl_id . resolver (db) ; let impl_data = db . impl_signature (impl_id) ; let mut ctx = TyLoweringContext :: new (db , & resolver , & impl_data . store , impl_id . into () , LifetimeElisionKind :: AnonymousCreateParameter { report_in_path : true } ,) ; let ty = ctx . lower_ty (impl_data . self_ty) ; assert ! (! ty . has_escaping_bound_vars ()) ; (EarlyBinder :: bind (ty) , create_diagnostics (ctx . diagnostics)) }
};
}
