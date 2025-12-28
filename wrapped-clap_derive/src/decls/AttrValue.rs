macro_rules! AttrValue {
    () => {
        # [derive (Clone)] # [allow (clippy :: large_enum_variant)] pub (crate) enum AttrValue { LitStr (LitStr) , Expr (Expr) , Call (Vec < Expr >) , }
    };
}

AttrValue!();