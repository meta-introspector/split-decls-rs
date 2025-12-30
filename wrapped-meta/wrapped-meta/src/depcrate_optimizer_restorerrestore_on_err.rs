// Generated macro for restore_on_err (function)
macro_rules! Depcrate_optimizer_restorerrestore_on_err {
() => {
// Module: crate::optimizer::restorer
// Provides: {"restore_on_err"}
// Dependencies: {}
pub fn restore_on_err (rule : OptimizedRule , rules : & HashMap < String , OptimizedExpr > ,) -> OptimizedRule { let OptimizedRule { name , ty , expr } = rule ; let expr = expr . map_bottom_up (| expr | wrap_branching_exprs (expr , rules)) ; OptimizedRule { name , ty , expr } }
};
}
