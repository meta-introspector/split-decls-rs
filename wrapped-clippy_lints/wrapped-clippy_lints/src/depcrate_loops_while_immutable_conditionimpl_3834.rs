// Generated macro for impl_3834 (impl)
macro_rules! Depcrate_loops_while_immutable_conditionimpl_3834 {
() => {
// Module: crate::loops::while_immutable_condition
// Provides: {"impl_3834"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for HasBreakOrReturnVisitor { type Result = ControlFlow < () > ; fn visit_expr (& mut self , expr : & 'tcx Expr < '_ >) -> ControlFlow < () > { match expr . kind { ExprKind :: Ret (_) | ExprKind :: Break (_ , _) => { return ControlFlow :: Break (()) ; } , _ => { } , } walk_expr (self , expr) } }
};
}
