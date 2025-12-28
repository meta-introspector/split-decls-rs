macro_rules! Index {
    () => {
        # [derive (Debug , Clone)] pub (crate) enum Index { Implicit (proc_macro2 :: TokenStream) , Explicit (proc_macro2 :: TokenStream) , }
    };
}

Index!()