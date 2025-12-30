// Generated macro for resolve_cond (function)
macro_rules! Depcrate_check_regionresolve_cond {
() => {
// Module: crate::check::region
// Provides: {"resolve_cond"}
// Dependencies: {}
# [doc = " Resolve a condition from an `if` expression or match guard so that it is a terminating scope"] # [doc = " if it doesn't contain `let` expressions."] fn resolve_cond < 'tcx > (visitor : & mut ScopeResolutionVisitor < 'tcx > , cond : & 'tcx hir :: Expr < 'tcx >) { let terminate = match cond . kind { hir :: ExprKind :: Let (_) => false , hir :: ExprKind :: Binary (source_map :: Spanned { node : hir :: BinOpKind :: And | hir :: BinOpKind :: Or , .. } , _ , _ ,) => false , _ => true , } ; resolve_expr (visitor , cond , terminate) ; }
};
}
