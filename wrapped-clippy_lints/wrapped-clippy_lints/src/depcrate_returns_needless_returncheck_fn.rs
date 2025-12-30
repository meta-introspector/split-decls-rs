// Generated macro for check_fn (function)
macro_rules! Depcrate_returns_needless_returncheck_fn {
() => {
// Module: crate::returns::needless_return
// Provides: {"check_fn"}
// Dependencies: {}
pub (super) fn check_fn < 'tcx > (cx : & LateContext < 'tcx > , kind : FnKind < 'tcx > , body : & 'tcx Body < 'tcx > , sp : Span) { if sp . from_expansion () { return ; } match kind { FnKind :: Closure => { let replacement = if let ExprKind :: Ret (None) = & body . value . kind { RetReplacement :: Block } else { RetReplacement :: Empty } ; check_final_expr (cx , body . value , vec ! [] , replacement , None) ; } , FnKind :: ItemFn (..) | FnKind :: Method (..) => { check_block_return (cx , & body . value . kind , sp , vec ! []) ; } , } }
};
}
