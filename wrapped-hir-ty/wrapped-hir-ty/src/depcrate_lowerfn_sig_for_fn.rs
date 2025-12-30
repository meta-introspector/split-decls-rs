// Generated macro for fn_sig_for_fn (function)
macro_rules! Depcrate_lowerfn_sig_for_fn {
() => {
// Module: crate::lower
// Provides: {"fn_sig_for_fn"}
// Dependencies: {}
fn fn_sig_for_fn < 'db > (db : & 'db dyn HirDatabase , def : FunctionId ,) -> EarlyBinder < 'db , PolyFnSig < 'db > > { let data = db . function_signature (def) ; let resolver = def . resolver (db) ; let interner = DbInterner :: new_with (db , Some (resolver . krate ()) , None) ; let mut ctx_params = TyLoweringContext :: new (db , & resolver , & data . store , def . into () , LifetimeElisionKind :: for_fn_params (& data) ,) ; let params = data . params . iter () . map (| & tr | ctx_params . lower_ty (tr)) ; let ret = match data . ret_type { Some (ret_type) => { let mut ctx_ret = TyLoweringContext :: new (db , & resolver , & data . store , def . into () , LifetimeElisionKind :: for_fn_ret (interner) ,) . with_impl_trait_mode (ImplTraitLoweringMode :: Opaque) ; ctx_ret . lower_ty (ret_type) } None => Ty :: new_tup (interner , & []) , } ; let inputs_and_output = Tys :: new_from_iter (interner , params . chain (Some (ret))) ; EarlyBinder :: bind (rustc_type_ir :: Binder :: dummy (FnSig { abi : data . abi . as_ref () . map_or (FnAbi :: Rust , FnAbi :: from_symbol) , c_variadic : data . is_varargs () , safety : if data . is_unsafe () { Safety :: Unsafe } else { Safety :: Safe } , inputs_and_output , })) }
};
}
