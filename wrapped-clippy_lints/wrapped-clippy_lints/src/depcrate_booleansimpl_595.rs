// Generated macro for impl_595 (impl)
macro_rules! Depcrate_booleansimpl_595 {
() => {
// Module: crate::booleans
// Provides: {"impl_595"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for NonminimalBoolVisitor < '_ , 'tcx > { fn visit_expr (& mut self , e : & 'tcx Expr < '_ >) { if ! e . span . from_expansion () { match & e . kind { ExprKind :: Binary (binop , _ , _) if binop . node == BinOpKind :: Or || binop . node == BinOpKind :: And && ! has_let_expr (e) => { self . bool_expr (e) ; } , ExprKind :: Unary (UnOp :: Not , inner) => { if let ExprKind :: Unary (UnOp :: Not , ex) = inner . kind && ! self . cx . typeck_results () . node_types () [ex . hir_id] . is_bool () { return ; } if self . cx . typeck_results () . node_types () [inner . hir_id] . is_bool () { self . bool_expr (e) ; } } , _ => { } , } } walk_expr (self , e) ; } }
};
}
