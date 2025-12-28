macro_rules! count_ident {
    () => {
        macro_rules ! count_ident { () => { 0 } ; ($ i0 : ident $ ($ i : ident) *) => { 1 + count_ident ! ($ ($ i) *) } ; }
    };
}

count_ident!();