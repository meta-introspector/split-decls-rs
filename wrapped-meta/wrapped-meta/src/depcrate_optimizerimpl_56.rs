// Generated macro for impl_56 (impl)
macro_rules! Depcrate_optimizerimpl_56 {
() => {
// Module: crate::optimizer
// Provides: {"impl_56"}
// Dependencies: {}
impl OptimizedExprTopDownIterator { # [doc = " Creates a new top down iterator from an `OptimizedExpr`."] pub fn new (expr : & OptimizedExpr) -> Self { let mut iter = OptimizedExprTopDownIterator { current : None , next : None , right_branches : vec ! [] , } ; iter . iterate_expr (expr . clone ()) ; iter } fn iterate_expr (& mut self , expr : OptimizedExpr) { self . current = Some (expr . clone ()) ; match expr { OptimizedExpr :: Seq (lhs , rhs) => { self . right_branches . push (* rhs) ; self . next = Some (* lhs) ; } OptimizedExpr :: Choice (lhs , rhs) => { self . right_branches . push (* rhs) ; self . next = Some (* lhs) ; } OptimizedExpr :: PosPred (expr) | OptimizedExpr :: NegPred (expr) | OptimizedExpr :: Rep (expr) | OptimizedExpr :: Opt (expr) | OptimizedExpr :: Push (expr) => { self . next = Some (* expr) ; } _ => { self . next = None ; } } } }
};
}
