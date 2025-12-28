macro_rules! deps {
    () => {
        ScopeResolutionVisitor!();
    };
}

macro_rules! resolve_expr {
    () => {
        deps!();
        # [tracing :: instrument (level = "debug" , skip (visitor))] fn resolve_expr < 'tcx > (visitor : & mut ScopeResolutionVisitor < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx > , terminating : bool ,) { let prev_cx = visitor . cx ; visitor . enter_node_scope_with_dtor (expr . hir_id . local_id , terminating) ; match expr . kind { hir :: ExprKind :: Binary (source_map :: Spanned { node : hir :: BinOpKind :: And | hir :: BinOpKind :: Or , .. } , left , right ,) => { let terminate_lhs = match left . kind { hir :: ExprKind :: Let (_) => false , hir :: ExprKind :: Binary (source_map :: Spanned { node : hir :: BinOpKind :: And | hir :: BinOpKind :: Or , .. } , .. ,) => false , _ => true , } ; let terminate_rhs = ! matches ! (right . kind , hir :: ExprKind :: Let (_)) ; resolve_expr (visitor , left , terminate_lhs) ; resolve_expr (visitor , right , terminate_rhs) ; } hir :: ExprKind :: Closure (& hir :: Closure { body , .. }) => { let body = visitor . tcx . hir_body (body) ; visitor . visit_body (body) ; } hir :: ExprKind :: AssignOp (_ , left_expr , right_expr) => { visitor . visit_expr (right_expr) ; visitor . visit_expr (left_expr) ; } hir :: ExprKind :: If (cond , then , Some (otherwise)) => { let expr_cx = visitor . cx ; let data = if expr . span . at_least_rust_2024 () { ScopeData :: IfThenRescope } else { ScopeData :: IfThen } ; visitor . enter_scope (Scope { local_id : then . hir_id . local_id , data }) ; visitor . cx . var_parent = (visitor . cx . parent , ScopeCompatibility :: FutureCompatible) ; resolve_cond (visitor , cond) ; resolve_expr (visitor , then , true) ; visitor . cx = expr_cx ; resolve_expr (visitor , otherwise , true) ; } hir :: ExprKind :: If (cond , then , None) => { let expr_cx = visitor . cx ; let data = if expr . span . at_least_rust_2024 () { ScopeData :: IfThenRescope } else { ScopeData :: IfThen } ; visitor . enter_scope (Scope { local_id : then . hir_id . local_id , data }) ; visitor . cx . var_parent = (visitor . cx . parent , ScopeCompatibility :: FutureCompatible) ; resolve_cond (visitor , cond) ; resolve_expr (visitor , then , true) ; visitor . cx = expr_cx ; } hir :: ExprKind :: Loop (body , _ , _ , _) => { resolve_block (visitor , body , true) ; } hir :: ExprKind :: DropTemps (expr) => { resolve_expr (visitor , expr , true) ; } _ => intravisit :: walk_expr (visitor , expr) , } visitor . cx = prev_cx ; }
    };
}

resolve_expr!();