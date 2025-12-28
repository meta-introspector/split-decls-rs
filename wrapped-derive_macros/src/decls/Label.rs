macro_rules! Label {
    () => {
        # [derive (Debug , Clone)] pub (crate) enum Label { Implicit (proc_macro2 :: TokenStream) , Const (proc_macro2 :: TokenStream) , Ident (proc_macro2 :: TokenStream) , }
    };
}

Label!();