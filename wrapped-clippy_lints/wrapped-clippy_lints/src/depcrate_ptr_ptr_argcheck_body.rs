// Generated macro for check_body (function)
macro_rules! Depcrate_ptr_ptr_argcheck_body {
() => {
// Module: crate::ptr::ptr_arg
// Provides: {"check_body"}
// Dependencies: {}
pub (super) fn check_body < 'tcx > (cx : & LateContext < 'tcx > , body : & Body < 'tcx > , item_id : OwnerId , sig : & FnSig < 'tcx > , is_trait_item : bool ,) { if ! matches ! (sig . header . abi , ExternAbi :: Rust) { return ; } let decl = sig . decl ; let sig = cx . tcx . fn_sig (item_id) . instantiate_identity () . skip_binder () ; let lint_args : Vec < _ > = check_fn_args (cx , sig , decl . inputs , body . params) . filter (| arg | ! is_trait_item || arg . mutability () == Mutability :: Not) . collect () ; let results = check_ptr_arg_usage (cx , body , & lint_args) ; for (result , args) in iter :: zip (& results , & lint_args) . filter (| (r , _) | ! r . skip) { span_lint_hir_and_then (cx , PTR_ARG , args . emission_id , args . span , args . build_msg () , | diag | { diag . multipart_suggestion ("change this to" , iter :: once ((args . span , format ! ("{}{}" , args . ref_prefix , args . deref_ty . display (cx)))) . chain (result . replacements . iter () . map (| r | { (r . expr_span , format ! ("{}{}" , r . self_span . get_source_text (cx) . unwrap () , r . replacement) ,) })) . collect () , Applicability :: Unspecified ,) ; }) ; } }
};
}
