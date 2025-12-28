macro_rules! deps {
    () => {
        ScopeResolutionVisitor!();
    };
}

macro_rules! resolve_stmt {
    () => {
        deps!();
        fn resolve_stmt < 'tcx > (visitor : & mut ScopeResolutionVisitor < 'tcx > , stmt : & 'tcx hir :: Stmt < 'tcx >) { let stmt_id = stmt . hir_id . local_id ; debug ! ("resolve_stmt(stmt.id={:?})" , stmt_id) ; if let hir :: StmtKind :: Let (LetStmt { super_ : Some (_) , .. }) = stmt . kind { intravisit :: walk_stmt (visitor , stmt) ; } else { let prev_parent = visitor . cx . parent ; visitor . enter_node_scope_with_dtor (stmt_id , true) ; intravisit :: walk_stmt (visitor , stmt) ; visitor . cx . parent = prev_parent ; } }
    };
}

resolve_stmt!()