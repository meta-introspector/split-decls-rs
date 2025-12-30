// Generated macro for impl_7588 (impl)
macro_rules! Depcrate_mutable_debug_assertionimpl_7588 {
() => {
// Module: crate::mutable_debug_assertion
// Provides: {"impl_7588"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for MutArgVisitor < '_ , 'tcx > { type NestedFilter = nested_filter :: OnlyBodies ; fn visit_expr (& mut self , expr : & 'tcx Expr < '_ >) { match expr . kind { ExprKind :: AddrOf (BorrowKind :: Ref , Mutability :: Mut , _) => { self . found = true ; return ; } , ExprKind :: If (..) => { self . found = true ; return ; } , ExprKind :: Path (_) => { if let Some (adj) = self . cx . typeck_results () . adjustments () . get (expr . hir_id) && adj . iter () . any (| a | matches ! (a . target . kind () , ty :: Ref (_ , _ , Mutability :: Mut))) { self . found = true ; return ; } } , ExprKind :: Match (_ , _ , MatchSource :: AwaitDesugar) => return , _ if ! self . found => self . expr_span = Some (expr . span) , _ => return , } walk_expr (self , expr) ; } fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } }
};
}
