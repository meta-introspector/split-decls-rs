macro_rules! deps {
    () => {
        ScopeResolutionVisitor!();
    };
}

macro_rules! resolve_cond {
    () => {
        deps!();
        # [doc = " Resolve a condition from an `if` expression or match guard so that it is a terminating scope"] # [doc = " if it doesn't contain `let` expressions."] fn resolve_cond < 'tcx > (visitor : & mut ScopeResolutionVisitor < 'tcx > , cond : & 'tcx hir :: Expr < 'tcx >) { let terminate = match cond . kind { hir :: ExprKind :: Let (_) => false , hir :: ExprKind :: Binary (source_map :: Spanned { node : hir :: BinOpKind :: And | hir :: BinOpKind :: Or , .. } , _ , _ ,) => false , _ => true , } ; resolve_expr (visitor , cond , terminate) ; }
    };
}

resolve_cond!();