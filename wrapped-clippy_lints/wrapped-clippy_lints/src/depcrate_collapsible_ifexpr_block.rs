// Generated macro for expr_block (function)
macro_rules! Depcrate_collapsible_ifexpr_block {
() => {
// Module: crate::collapsible_if
// Provides: {"expr_block"}
// Dependencies: {}
# [doc = " If `block` is a block with either one expression or a statement containing an expression,"] # [doc = " return the expression. We don't peel blocks recursively, as extra blocks might be intentional."] fn expr_block < 'tcx > (block : & Block < 'tcx >) -> Option < & 'tcx Expr < 'tcx > > { match (block . stmts , block . expr) { ([] , expr) => expr , ([stmt] , None) if let StmtKind :: Semi (expr) = stmt . kind => Some (expr) , _ => None , } }
};
}
