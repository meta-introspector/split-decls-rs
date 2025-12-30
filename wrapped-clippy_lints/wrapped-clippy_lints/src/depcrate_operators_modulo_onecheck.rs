// Generated macro for check (function)
macro_rules! Depcrate_operators_modulo_onecheck {
() => {
// Module: crate::operators::modulo_one
// Provides: {"check"}
// Dependencies: {}
pub (crate) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , op : BinOpKind , right : & Expr < '_ >) { if op == BinOpKind :: Rem { if is_integer_const (cx , right , 1) { span_lint (cx , MODULO_ONE , expr . span , "any number modulo 1 will be 0") ; } if let ty :: Int (ity) = cx . typeck_results () . expr_ty (right) . kind () && is_integer_const (cx , right , unsext (cx . tcx , - 1 , * ity)) { span_lint (cx , MODULO_ONE , expr . span , "any number modulo -1 will panic/overflow or result in 0" ,) ; } } }
};
}
