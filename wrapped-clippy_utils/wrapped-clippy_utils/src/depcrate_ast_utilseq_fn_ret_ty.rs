// Generated macro for eq_fn_ret_ty (function)
macro_rules! Depcrate_ast_utilseq_fn_ret_ty {
() => {
// Module: crate::ast_utils
// Provides: {"eq_fn_ret_ty"}
// Dependencies: {}
pub fn eq_fn_ret_ty (l : & FnRetTy , r : & FnRetTy) -> bool { match (l , r) { (FnRetTy :: Default (_) , FnRetTy :: Default (_)) => true , (FnRetTy :: Ty (l) , FnRetTy :: Ty (r)) => eq_ty (l , r) , _ => false , } }
};
}
