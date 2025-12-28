macro_rules! deps {
    () => {
        SelfToDbRewriter!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl VisitMut for SelfToDbRewriter { fn visit_expr_path_mut (& mut self , i : & mut syn :: ExprPath) { if i . path . is_ident ("self") { i . path = parse_quote_spanned ! (i . path . span () => db) ; } } }
    };
}

impl_40!();