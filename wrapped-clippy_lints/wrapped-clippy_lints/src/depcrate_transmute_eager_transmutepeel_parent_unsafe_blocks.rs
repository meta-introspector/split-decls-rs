// Generated macro for peel_parent_unsafe_blocks (function)
macro_rules! Depcrate_transmute_eager_transmutepeel_parent_unsafe_blocks {
() => {
// Module: crate::transmute::eager_transmute
// Provides: {"peel_parent_unsafe_blocks"}
// Dependencies: {}
fn peel_parent_unsafe_blocks < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> Option < & 'tcx Expr < 'tcx > > { for (_ , parent) in cx . tcx . hir_parent_iter (expr . hir_id) { match parent { Node :: Block (_) => { } , Node :: Expr (e) if let ExprKind :: Block (..) = e . kind => { } , Node :: Expr (e) => return Some (e) , _ => break , } } None }
};
}
