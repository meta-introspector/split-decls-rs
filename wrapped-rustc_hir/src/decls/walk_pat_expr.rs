macro_rules! deps {
    () => {
        Path!();
        Visitor!();
        PatExprKind!();
        PatExpr!();
        Lit!();
        ConstBlock!();
    };
}

macro_rules! walk_pat_expr {
    () => {
        deps!();
        pub fn walk_pat_expr < 'v , V : Visitor < 'v > > (visitor : & mut V , expr : & 'v PatExpr < 'v >) -> V :: Result { let PatExpr { hir_id , span , kind } = expr ; try_visit ! (visitor . visit_id (* hir_id)) ; match kind { PatExprKind :: Lit { lit , negated } => visitor . visit_lit (* hir_id , * lit , * negated) , PatExprKind :: ConstBlock (c) => visitor . visit_inline_const (c) , PatExprKind :: Path (qpath) => visitor . visit_qpath (qpath , * hir_id , * span) , } }
    };
}

walk_pat_expr!()