// Generated macro for is_in_method_chain (function)
macro_rules! Depcrate_dereferenceis_in_method_chain {
() => {
// Module: crate::dereference
// Provides: {"is_in_method_chain"}
// Dependencies: {}
fn is_in_method_chain < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < 'tcx >) -> bool { if let ExprKind :: MethodCall (_ , recv , _ , _) = e . kind && matches ! (recv . kind , ExprKind :: MethodCall (..)) { return true ; } if let Some (parent) = get_parent_expr (cx , e) && parent . span . eq_ctxt (e . span) { match parent . kind { ExprKind :: Call (child , _) | ExprKind :: MethodCall (_ , child , _ , _) | ExprKind :: Index (child , _ , _) if child . hir_id == e . hir_id => { true } , ExprKind :: Match (.. , MatchSource :: TryDesugar (_) | MatchSource :: AwaitDesugar) | ExprKind :: Field (_ , _) => true , _ => false , } } else { false } }
};
}
