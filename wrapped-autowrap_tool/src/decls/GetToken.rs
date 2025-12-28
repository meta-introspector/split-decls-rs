macro_rules! GetToken {
    () => {
        macro_rules ! GetToken { ($ token : tt) => { syn :: token ::$ token :: new (proc_macro2 :: Span :: call_site ()) } ; }
    };
}

GetToken!();