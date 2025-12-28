macro_rules! match_head_needs_bracket {
    () => {
        fn match_head_needs_bracket (tcx : TyCtxt < '_ > , expr : & hir :: Expr < '_ >) -> bool { expr_parent_is_else (tcx , expr . hir_id) && matches ! (expr . kind , hir :: ExprKind :: If (..)) }
    };
}

match_head_needs_bracket!()