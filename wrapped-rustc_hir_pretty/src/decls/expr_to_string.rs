macro_rules! deps {
    () => {
        PpAnn!();
    };
}

macro_rules! expr_to_string {
    () => {
        deps!();
        pub fn expr_to_string (ann : & dyn PpAnn , pat : & hir :: Expr < '_ >) -> String { to_string (ann , | s | s . print_expr (pat)) }
    };
}

expr_to_string!();