macro_rules! deps {
    () => {
        LateContext!();
        UnusedAllocationDiag!();
        UnusedAllocationMutDiag!();
    };
}

macro_rules! impl_862 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for UnusedAllocation { fn check_expr (& mut self , cx : & LateContext < '_ > , e : & hir :: Expr < '_ >) { match e . kind { hir :: ExprKind :: Call (path_expr , [_]) if let hir :: ExprKind :: Path (qpath) = & path_expr . kind && let Some (did) = cx . qpath_res (qpath , path_expr . hir_id) . opt_def_id () && cx . tcx . is_diagnostic_item (sym :: box_new , did) => { } _ => return , } for adj in cx . typeck_results () . expr_adjustments (e) { if let adjustment :: Adjust :: Borrow (adjustment :: AutoBorrow :: Ref (m)) = adj . kind { match m { adjustment :: AutoBorrowMutability :: Not => { cx . emit_span_lint (UNUSED_ALLOCATION , e . span , UnusedAllocationDiag) ; } adjustment :: AutoBorrowMutability :: Mut { .. } => { cx . emit_span_lint (UNUSED_ALLOCATION , e . span , UnusedAllocationMutDiag) ; } } ; } } } }
    };
}

impl_862!()