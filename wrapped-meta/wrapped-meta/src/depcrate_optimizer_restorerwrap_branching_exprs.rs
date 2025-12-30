// Generated macro for wrap_branching_exprs (function)
macro_rules! Depcrate_optimizer_restorerwrap_branching_exprs {
() => {
// Module: crate::optimizer::restorer
// Provides: {"wrap_branching_exprs"}
// Dependencies: {}
fn wrap_branching_exprs (expr : OptimizedExpr , rules : & HashMap < String , OptimizedExpr > ,) -> OptimizedExpr { match expr { OptimizedExpr :: Opt (expr) => { if child_modifies_state (& expr , rules , & mut HashMap :: new ()) { OptimizedExpr :: Opt (Box :: new (OptimizedExpr :: RestoreOnErr (expr))) } else { OptimizedExpr :: Opt (expr) } } OptimizedExpr :: Choice (lhs , rhs) => { let wrapped_lhs = if child_modifies_state (& lhs , rules , & mut HashMap :: new ()) { Box :: new (OptimizedExpr :: RestoreOnErr (lhs)) } else { lhs } ; let wrapped_rhs = if child_modifies_state (& rhs , rules , & mut HashMap :: new ()) { Box :: new (OptimizedExpr :: RestoreOnErr (rhs)) } else { rhs } ; OptimizedExpr :: Choice (wrapped_lhs , wrapped_rhs) } OptimizedExpr :: Rep (expr) => { if child_modifies_state (& expr , rules , & mut HashMap :: new ()) { OptimizedExpr :: Rep (Box :: new (OptimizedExpr :: RestoreOnErr (expr))) } else { OptimizedExpr :: Rep (expr) } } _ => expr , } }
};
}
