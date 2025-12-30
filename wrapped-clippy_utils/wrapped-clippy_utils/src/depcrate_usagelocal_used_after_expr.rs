// Generated macro for local_used_after_expr (function)
macro_rules! Depcrate_usagelocal_used_after_expr {
() => {
// Module: crate::usage
// Provides: {"local_used_after_expr"}
// Dependencies: {}
pub fn local_used_after_expr (cx : & LateContext < '_ > , local_id : HirId , after : & Expr < '_ >) -> bool { let Some (block) = utils :: get_enclosing_block (cx , local_id) else { return false ; } ; let loop_start = get_enclosing_loop_or_multi_call_closure (cx , after) . map (| e | e . hir_id) ; let mut past_expr = false ; for_each_expr (cx , block , | e | { if past_expr { if e . res_local_id () == Some (local_id) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (Descend :: Yes) } } else if e . hir_id == after . hir_id { past_expr = true ; ControlFlow :: Continue (Descend :: No) } else { past_expr = Some (e . hir_id) == loop_start ; ControlFlow :: Continue (Descend :: Yes) } }) . is_some () }
};
}
