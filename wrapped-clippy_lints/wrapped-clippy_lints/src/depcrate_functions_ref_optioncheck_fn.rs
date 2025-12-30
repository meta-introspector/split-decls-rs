// Generated macro for check_fn (function)
macro_rules! Depcrate_functions_ref_optioncheck_fn {
() => {
// Module: crate::functions::ref_option
// Provides: {"check_fn"}
// Dependencies: {}
# [expect (clippy :: too_many_arguments)] pub (crate) fn check_fn < 'a > (cx : & LateContext < 'a > , kind : FnKind < 'a > , decl : & FnDecl < 'a > , span : Span , hir_id : HirId , def_id : LocalDefId , body : & hir :: Body < 'a > , avoid_breaking_exported_api : bool ,) { if avoid_breaking_exported_api && cx . effective_visibilities . is_exported (def_id) { return ; } if span . in_external_macro (cx . sess () . source_map ()) { return ; } if let FnKind :: Closure = kind { let inputs_output_span = if let hir :: FnRetTy :: Return (out_ty) = & decl . output { if decl . inputs . is_empty () { out_ty . span } else { span . with_hi (out_ty . span . hi ()) } } else if let (Some (first) , Some (last)) = (decl . inputs . first () , decl . inputs . last ()) { first . span . to (last . span) } else { return ; } ; let ty :: Closure (_ , args) = cx . typeck_results () . expr_ty (body . value) . kind () else { return ; } ; let sig = args . as_closure () . sig () . skip_binder () ; if is_from_proc_macro (cx , & (& kind , body , hir_id , span)) { return ; } check_fn_sig (cx , decl , inputs_output_span , sig) ; } else if ! is_trait_impl_item (cx , hir_id) { let sig = cx . tcx . fn_sig (def_id) . instantiate_identity () . skip_binder () ; if is_from_proc_macro (cx , & (& kind , body , hir_id , span)) { return ; } check_fn_sig (cx , decl , span , sig) ; } }
};
}
