macro_rules! deps {
    () => {
        Expr!();
    };
}

macro_rules! walk_filter_map_expr {
    () => {
        deps!();
        pub fn walk_filter_map_expr < T : MutVisitor > (vis : & mut T , mut e : Box < Expr >) -> Option < Box < Expr > > { vis . visit_expr (& mut e) ; Some (e) }
    };
}

walk_filter_map_expr!();