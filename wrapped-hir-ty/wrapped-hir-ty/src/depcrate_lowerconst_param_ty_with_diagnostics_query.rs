// Generated macro for const_param_ty_with_diagnostics_query (function)
macro_rules! Depcrate_lowerconst_param_ty_with_diagnostics_query {
() => {
// Module: crate::lower
// Provides: {"const_param_ty_with_diagnostics_query"}
// Dependencies: {}
pub (crate) fn const_param_ty_with_diagnostics_query < 'db > (db : & 'db dyn HirDatabase , def : ConstParamId ,) -> (Ty < 'db > , Diagnostics) { let (parent_data , store) = db . generic_params_and_store (def . parent ()) ; let data = & parent_data [def . local_id ()] ; let resolver = def . parent () . resolver (db) ; let interner = DbInterner :: new_with (db , Some (resolver . krate ()) , None) ; let mut ctx = TyLoweringContext :: new (db , & resolver , & store , def . parent () , LifetimeElisionKind :: AnonymousReportError ,) ; let ty = match data { TypeOrConstParamData :: TypeParamData (_) => { never ! () ; Ty :: new_error (interner , ErrorGuaranteed) } TypeOrConstParamData :: ConstParamData (d) => ctx . lower_ty (d . ty) , } ; (ty , create_diagnostics (ctx . diagnostics)) }
};
}
