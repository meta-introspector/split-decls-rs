macro_rules! deps {
    () => {
        ExprField!();
        Visitor!();
    };
}

macro_rules! walk_expr_field {
    () => {
        deps!();
        pub fn walk_expr_field < 'v , V : Visitor < 'v > > (visitor : & mut V , field : & 'v ExprField < 'v >) -> V :: Result { let ExprField { hir_id , ident , expr , span : _ , is_shorthand : _ } = field ; try_visit ! (visitor . visit_id (* hir_id)) ; try_visit ! (visitor . visit_ident (* ident)) ; visitor . visit_expr (* expr) }
    };
}

walk_expr_field!()