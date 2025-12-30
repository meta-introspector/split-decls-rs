// Generated macro for clone_or_copy_needed (function)
macro_rules! Depcrate_methods_utilsclone_or_copy_needed {
() => {
// Module: crate::methods::utils
// Provides: {"clone_or_copy_needed"}
// Dependencies: {}
# [doc = " The core logic of `check_for_loop_iter` in `unnecessary_iter_cloned.rs`, this function wraps a"] # [doc = " use of `CloneOrCopyVisitor`."] pub (super) fn clone_or_copy_needed < 'tcx > (cx : & LateContext < 'tcx > , pat : & Pat < 'tcx > , body : & 'tcx Expr < 'tcx > ,) -> (bool , Vec < (Span , String) >) { let mut visitor = CloneOrCopyVisitor { cx , binding_hir_ids : pat_bindings (pat) , clone_or_copy_needed : false , references_to_binding : Vec :: new () , } ; visitor . visit_expr (body) ; (visitor . clone_or_copy_needed , visitor . references_to_binding) }
};
}
