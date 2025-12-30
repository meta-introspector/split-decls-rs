// Generated macro for impl_7737 (impl)
macro_rules! Depcrate_needless_for_eachimpl_7737 {
() => {
// Module: crate::needless_for_each
// Provides: {"impl_7737"}
// Dependencies: {}
impl Visitor < '_ > for RetCollector { fn visit_expr (& mut self , expr : & Expr < '_ >) { match expr . kind { ExprKind :: Ret (..) => { if self . loop_depth > 0 && ! self . ret_in_loop { self . ret_in_loop = true ; } self . spans . push (expr . span) ; } , ExprKind :: Loop (..) => { self . loop_depth += 1 ; walk_expr (self , expr) ; self . loop_depth -= 1 ; return ; } , _ => { } , } walk_expr (self , expr) ; } }
};
}
