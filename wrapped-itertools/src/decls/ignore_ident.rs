macro_rules! ignore_ident {
    () => {
        macro_rules ! ignore_ident { ($ id : ident , $ ($ t : tt) *) => { $ ($ t) * } ; }
    };
}

ignore_ident!();