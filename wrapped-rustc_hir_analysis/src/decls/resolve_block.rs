macro_rules! deps {
    () => {
        ScopeResolutionVisitor!();
    };
}

macro_rules! resolve_block {
    () => {
        deps!();
        fn resolve_block < 'tcx > (visitor : & mut ScopeResolutionVisitor < 'tcx > , blk : & 'tcx hir :: Block < 'tcx > , terminating : bool ,) { debug ! ("resolve_block(blk.hir_id={:?})" , blk . hir_id) ; let prev_cx = visitor . cx ; visitor . enter_node_scope_with_dtor (blk . hir_id . local_id , terminating) ; visitor . cx . var_parent = (visitor . cx . parent , ScopeCompatibility :: FutureCompatible) ; { for (i , statement) in blk . stmts . iter () . enumerate () { match statement . kind { hir :: StmtKind :: Let (LetStmt { els : Some (els) , .. }) => { let mut prev_cx = visitor . cx ; visitor . enter_scope (Scope { local_id : blk . hir_id . local_id , data : ScopeData :: Remainder (FirstStatementIndex :: new (i)) , }) ; visitor . cx . var_parent = (visitor . cx . parent , ScopeCompatibility :: FutureCompatible) ; visitor . visit_stmt (statement) ; mem :: swap (& mut prev_cx , & mut visitor . cx) ; resolve_block (visitor , els , true) ; visitor . cx = prev_cx ; } hir :: StmtKind :: Let (..) => { visitor . enter_scope (Scope { local_id : blk . hir_id . local_id , data : ScopeData :: Remainder (FirstStatementIndex :: new (i)) , }) ; visitor . cx . var_parent = (visitor . cx . parent , ScopeCompatibility :: FutureCompatible) ; visitor . visit_stmt (statement) } hir :: StmtKind :: Item (..) => { } hir :: StmtKind :: Expr (..) | hir :: StmtKind :: Semi (..) => visitor . visit_stmt (statement) , } } if let Some (tail_expr) = blk . expr { let local_id = tail_expr . hir_id . local_id ; let edition = blk . span . edition () ; let terminating = edition . at_least_rust_2024 () ; if ! terminating && ! visitor . tcx . lints_that_dont_need_to_run (()) . contains (& lint :: LintId :: of (lint :: builtin :: TAIL_EXPR_DROP_ORDER)) { visitor . scope_tree . backwards_incompatible_scope . insert (local_id , Scope { local_id , data : ScopeData :: Node }) ; } resolve_expr (visitor , tail_expr , terminating) ; } } visitor . cx = prev_cx ; }
    };
}

resolve_block!()