macro_rules! deps {
    () => {
        Item!();
        Expr!();
        MacCall!();
        MacCallStmt!();
        StmtKind!();
    };
}

macro_rules! walk_flat_map_stmt_kind {
    () => {
        deps!();
        fn walk_flat_map_stmt_kind < T : MutVisitor > (vis : & mut T , kind : StmtKind) -> SmallVec < [StmtKind ; 1] > { match kind { StmtKind :: Let (mut local) => smallvec ! [StmtKind :: Let ({ vis . visit_local (& mut local) ; local })] , StmtKind :: Item (item) => vis . flat_map_item (item) . into_iter () . map (StmtKind :: Item) . collect () , StmtKind :: Expr (expr) => vis . filter_map_expr (expr) . into_iter () . map (StmtKind :: Expr) . collect () , StmtKind :: Semi (expr) => vis . filter_map_expr (expr) . into_iter () . map (StmtKind :: Semi) . collect () , StmtKind :: Empty => smallvec ! [StmtKind :: Empty] , StmtKind :: MacCall (mut mac) => { let MacCallStmt { mac : mac_ , style : _ , attrs , tokens : _ } = mac . deref_mut () ; for attr in attrs { vis . visit_attribute (attr) ; } vis . visit_mac_call (mac_) ; smallvec ! [StmtKind :: MacCall (mac)] } } }
    };
}

walk_flat_map_stmt_kind!();