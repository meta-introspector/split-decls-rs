// Generated macro for impl_13 (impl)
macro_rules! Depcrate_astimpl_13 {
() => {
// Module: crate::ast
// Provides: {"impl_13"}
// Dependencies: {}
impl ExprTopDownIterator { # [doc = " Constructs a top-down iterator from the expression."] pub fn new (expr : & Expr) -> Self { let mut iter = ExprTopDownIterator { current : None , next : None , right_branches : vec ! [] , } ; iter . iterate_expr (expr . clone ()) ; iter } fn iterate_expr (& mut self , expr : Expr) { self . current = Some (expr . clone ()) ; match expr { Expr :: Seq (lhs , rhs) => { self . right_branches . push (* rhs) ; self . next = Some (* lhs) ; } Expr :: Choice (lhs , rhs) => { self . right_branches . push (* rhs) ; self . next = Some (* lhs) ; } Expr :: PosPred (expr) | Expr :: NegPred (expr) | Expr :: Rep (expr) | Expr :: RepOnce (expr) | Expr :: RepExact (expr , _) | Expr :: RepMin (expr , _) | Expr :: RepMax (expr , _) | Expr :: RepMinMax (expr , ..) | Expr :: Opt (expr) | Expr :: Push (expr) => { self . next = Some (* expr) ; } # [cfg (feature = "grammar-extras")] Expr :: NodeTag (expr , _) => { self . next = Some (* expr) ; } _ => { self . next = None ; } } } }
};
}
