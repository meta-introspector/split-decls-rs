// Generated macro for prepare_receiver_sugg (function)
macro_rules! Depcrate_floating_point_arithmeticprepare_receiver_sugg {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"prepare_receiver_sugg"}
// Dependencies: {}
fn prepare_receiver_sugg < 'a > (cx : & LateContext < '_ > , mut expr : & 'a Expr < 'a >) -> Sugg < 'a > { let mut suggestion = Sugg :: hir (cx , expr , "..") ; if let ExprKind :: Unary (UnOp :: Neg , inner_expr) = & expr . kind { expr = inner_expr ; } if let ty :: Float (float_ty) = cx . typeck_results () . expr_ty (expr) . kind () && let ExprKind :: Lit (lit) = & expr . kind && let ast :: LitKind :: Float (sym , ast :: LitFloatType :: Unsuffixed) = lit . node { let op = format ! ("{suggestion}{}{}" , if sym . as_str () . ends_with ('.') { "0" } else { "" } , float_ty . name_str ()) . into () ; suggestion = match suggestion { Sugg :: MaybeParen (_) | Sugg :: UnOp (UnOp :: Neg , _) => Sugg :: MaybeParen (op) , _ => Sugg :: NonParen (op) , } ; } suggestion . maybe_paren () }
};
}
