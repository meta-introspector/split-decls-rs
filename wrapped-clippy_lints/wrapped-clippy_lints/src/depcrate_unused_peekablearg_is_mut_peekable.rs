// Generated macro for arg_is_mut_peekable (function)
macro_rules! Depcrate_unused_peekablearg_is_mut_peekable {
() => {
// Module: crate::unused_peekable
// Provides: {"arg_is_mut_peekable"}
// Dependencies: {}
fn arg_is_mut_peekable (cx : & LateContext < '_ > , arg : & Expr < '_ >) -> bool { if let Some (ty) = cx . typeck_results () . expr_ty_opt (arg) && let (ty , _ , None | Some (Mutability :: Mut)) = peel_and_count_ty_refs (ty) && ty . is_diag_item (cx , sym :: IterPeekable) { true } else { false } }
};
}
