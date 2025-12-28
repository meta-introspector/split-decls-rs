macro_rules! Lit {
    () => {
        # [doc = " A literal."] pub type Lit = Spanned < LitKind > ;
    };
}

Lit!();