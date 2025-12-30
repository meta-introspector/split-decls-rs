// Generated macro for check_trait_item (function)
macro_rules! Depcrate_ptr_ptr_argcheck_trait_item {
() => {
// Module: crate::ptr::ptr_arg
// Provides: {"check_trait_item"}
// Dependencies: {}
pub (super) fn check_trait_item < 'tcx > (cx : & LateContext < 'tcx > , item_id : OwnerId , sig : & FnSig < 'tcx >) { if ! matches ! (sig . header . abi , ExternAbi :: Rust) { return ; } for arg in check_fn_args (cx , cx . tcx . fn_sig (item_id) . instantiate_identity () . skip_binder () , sig . decl . inputs , & [] ,) . filter (| arg | arg . mutability () == Mutability :: Not) { span_lint_hir_and_then (cx , PTR_ARG , arg . emission_id , arg . span , arg . build_msg () , | diag | { diag . span_suggestion (arg . span , "change this to" , format ! ("{}{}" , arg . ref_prefix , arg . deref_ty . display (cx)) , Applicability :: Unspecified ,) ; }) ; } }
};
}
