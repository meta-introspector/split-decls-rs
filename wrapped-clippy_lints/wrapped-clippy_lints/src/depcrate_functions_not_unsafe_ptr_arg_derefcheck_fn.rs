// Generated macro for check_fn (function)
macro_rules! Depcrate_functions_not_unsafe_ptr_arg_derefcheck_fn {
() => {
// Module: crate::functions::not_unsafe_ptr_arg_deref
// Provides: {"check_fn"}
// Dependencies: {}
pub (super) fn check_fn < 'tcx > (cx : & LateContext < 'tcx > , kind : intravisit :: FnKind < 'tcx > , decl : & 'tcx hir :: FnDecl < 'tcx > , body : & 'tcx hir :: Body < 'tcx > , def_id : LocalDefId ,) { let safety = match kind { intravisit :: FnKind :: ItemFn (_ , _ , header) => header . safety () , intravisit :: FnKind :: Method (_ , sig) => sig . header . safety () , intravisit :: FnKind :: Closure => return , } ; check_raw_ptr (cx , safety , decl , body , def_id) ; }
};
}
