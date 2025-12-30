// Generated macro for handle_uninit_vec_pair (function)
macro_rules! Depcrate_uninit_vechandle_uninit_vec_pair {
() => {
// Module: crate::uninit_vec
// Provides: {"handle_uninit_vec_pair"}
// Dependencies: {}
fn handle_uninit_vec_pair < 'tcx > (cx : & LateContext < 'tcx > , maybe_init_or_reserve : & 'tcx Stmt < 'tcx > , maybe_set_len : & 'tcx Expr < 'tcx > ,) { if let Some (vec) = extract_init_or_reserve_target (cx , maybe_init_or_reserve) && let Some ((set_len_self , call_span)) = extract_set_len_self (cx , maybe_set_len) && vec . location . eq_expr (cx , set_len_self) && let ty :: Ref (_ , vec_ty , _) = cx . typeck_results () . expr_ty_adjusted (set_len_self) . kind () && let ty :: Adt (_ , args) = vec_ty . kind () && ! is_lint_allowed (cx , UNINIT_VEC , maybe_set_len . hir_id) { if vec . has_capacity () { if ! is_uninit_value_valid_for_ty (cx , args . type_at (0)) { span_lint_and_help (cx , UNINIT_VEC , vec ! [call_span , maybe_init_or_reserve . span] , "calling `set_len()` immediately after reserving a buffer creates uninitialized values" , None , "initialize the buffer or wrap the content in `MaybeUninit`" ,) ; } } else { span_lint (cx , UNINIT_VEC , vec ! [call_span , maybe_init_or_reserve . span] , "calling `set_len()` on empty `Vec` creates out-of-bound values" ,) ; } } }
};
}
