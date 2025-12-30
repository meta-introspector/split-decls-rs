// Generated macro for impl_578 (impl)
macro_rules! Depcrate_booleansimpl_578 {
() => {
// Module: crate::booleans
// Provides: {"impl_578"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for NonminimalBool { fn check_fn (& mut self , cx : & LateContext < 'tcx > , _ : FnKind < 'tcx > , _ : & 'tcx FnDecl < '_ > , body : & 'tcx Body < '_ > , _ : Span , _ : LocalDefId ,) { NonminimalBoolVisitor { cx , msrv : self . msrv } . visit_body (body) ; } fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { match expr . kind { ExprKind :: Binary (op , left , right) if matches ! (op . node , BinOpKind :: Eq | BinOpKind :: Ne) => { check_inverted_bool_in_condition (cx , expr . span , op . node , left , right) ; } , _ => { } , } } }
};
}
