// Generated macro for expr_can_be_pat (function)
macro_rules! Depcrate_matches_redundant_guardsexpr_can_be_pat {
() => {
// Module: crate::matches::redundant_guards
// Provides: {"expr_can_be_pat"}
// Dependencies: {}
# [doc = " Checks if the given `Expr` can also be represented as a `Pat`."] fn expr_can_be_pat (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { for_each_expr_without_closures (expr , | expr | { if match expr . kind { ExprKind :: Call (c , ..) if let ExprKind :: Path (qpath) = c . kind => { matches ! (cx . qpath_res (& qpath , c . hir_id) , Res :: Def (DefKind :: Ctor (..) , ..)) } , ExprKind :: Path (qpath) => { matches ! (cx . qpath_res (& qpath , expr . hir_id) , Res :: Def (DefKind :: Struct | DefKind :: Enum | DefKind :: Ctor (..) , ..) ,) } , ExprKind :: AddrOf (..) | ExprKind :: Array (..) | ExprKind :: Tup (..) | ExprKind :: Struct (..) | ExprKind :: Unary (UnOp :: Neg , _) => true , ExprKind :: Lit (lit) if ! matches ! (lit . node , LitKind :: CStr (..)) => true , _ => false , } { return ControlFlow :: Continue (()) ; } ControlFlow :: Break (()) }) . is_none () }
};
}
