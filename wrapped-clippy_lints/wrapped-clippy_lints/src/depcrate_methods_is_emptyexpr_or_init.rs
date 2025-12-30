// Generated macro for expr_or_init (function)
macro_rules! Depcrate_methods_is_emptyexpr_or_init {
() => {
// Module: crate::methods::is_empty
// Provides: {"expr_or_init"}
// Dependencies: {}
# [doc = " Similar to [`clippy_utils::expr_or_init`], but does not go up the chain if the initialization"] # [doc = " value depends on a `#[cfg(…)]` directive."] fn expr_or_init < 'a , 'b , 'tcx : 'b > (cx : & LateContext < 'tcx > , mut expr : & 'a Expr < 'b >) -> & 'a Expr < 'b > { while let Some (init) = expr . res_local_id () . and_then (| id | find_binding_init (cx , id)) . filter (| init | cx . typeck_results () . expr_adjustments (init) . is_empty ()) . filter (| init | ! is_under_cfg (cx , init . hir_id)) { expr = init ; } expr }
};
}
