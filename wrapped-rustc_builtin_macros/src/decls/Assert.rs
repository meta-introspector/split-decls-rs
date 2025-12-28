macro_rules! Assert {
    () => {
        struct Assert { cond_expr : Box < Expr > , custom_message : Option < TokenStream > , }
    };
}

Assert!();