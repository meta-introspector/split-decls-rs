// Generated macro for impl_392 (impl)
macro_rules! Depcrate_expr_storeimpl_392 {
() => {
// Module: crate::expr_store
// Provides: {"impl_392"}
// Dependencies: {}
impl Index < ExprId > for ExpressionStore { type Output = Expr ; # [inline] fn index (& self , expr : ExprId) -> & Expr { & self . assert_expr_only () . exprs [expr] } }
};
}
