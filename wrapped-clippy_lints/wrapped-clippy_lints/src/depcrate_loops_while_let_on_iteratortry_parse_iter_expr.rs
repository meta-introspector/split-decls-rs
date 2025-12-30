// Generated macro for try_parse_iter_expr (function)
macro_rules! Depcrate_loops_while_let_on_iteratortry_parse_iter_expr {
() => {
// Module: crate::loops::while_let_on_iterator
// Provides: {"try_parse_iter_expr"}
// Dependencies: {}
# [doc = " Parses any expression to find out which field of which variable is used. Will return `None` if"] # [doc = " the expression might have side effects."] fn try_parse_iter_expr (cx : & LateContext < '_ > , mut e : & Expr < '_ >) -> Option < IterExpr > { let mut fields = Vec :: new () ; let mut can_move = true ; loop { if cx . typeck_results () . expr_adjustments (e) . iter () . any (| a | matches ! (a . kind , Adjust :: Deref (Some (..)))) { can_move = false ; fields . clear () ; } match e . kind { ExprKind :: Path (ref path) => { break Some (IterExpr { fields , path : cx . qpath_res (path , e . hir_id) , can_move , }) ; } , ExprKind :: Field (base , name) => { fields . push (name . name) ; e = base ; } , ExprKind :: Unary (UnOp :: Deref , base) if cx . typeck_results () . expr_ty (base) . is_ref () => e = base , ExprKind :: Index (base , idx , _) if ! idx . can_have_side_effects () => { can_move = false ; fields . clear () ; e = base ; } , ExprKind :: Unary (UnOp :: Deref , base) => { can_move = false ; fields . clear () ; e = base ; } , ExprKind :: DropTemps (base) | ExprKind :: AddrOf (_ , _ , base) | ExprKind :: Type (base , _) => e = base , _ => break None , } } }
};
}
