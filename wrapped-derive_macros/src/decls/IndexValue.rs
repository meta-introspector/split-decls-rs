macro_rules! IndexValue {
    () => {
        # [derive (Debug , Clone)] pub (crate) enum IndexValue { Const (isize) , Ident (proc_macro2 :: TokenStream) , }
    };
}

IndexValue!();