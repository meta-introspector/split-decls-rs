// Generated macro for impl_3837 (impl)
macro_rules! Depcrate_loops_while_immutable_conditionimpl_3837 {
() => {
// Module: crate::loops::while_immutable_condition
// Provides: {"impl_3837"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for VarCollectorVisitor < '_ , 'tcx > { type Result = ControlFlow < () > ; fn visit_expr (& mut self , ex : & 'tcx Expr < '_ >) -> Self :: Result { match ex . kind { ExprKind :: Path (_) => { self . insert_def_id (ex) ; ControlFlow :: Continue (()) } , ExprKind :: Call (..) | ExprKind :: MethodCall (..) => ControlFlow :: Break (()) , _ => walk_expr (self , ex) , } } }
};
}
