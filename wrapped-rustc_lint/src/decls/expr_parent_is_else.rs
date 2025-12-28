macro_rules! expr_parent_is_else {
    () => {
        fn expr_parent_is_else (tcx : TyCtxt < '_ > , hir_id : hir :: HirId) -> bool { let Some ((_ , hir :: Node :: Expr (expr))) = tcx . hir_parent_iter (hir_id) . next () else { return false ; } ; let hir :: ExprKind :: If (_cond , _conseq , Some (alt)) = expr . kind else { return false } ; alt . hir_id == hir_id }
    };
}

expr_parent_is_else!()