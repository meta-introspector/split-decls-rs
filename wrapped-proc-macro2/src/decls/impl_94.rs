macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        # [cfg (feature = "proc-macro")] impl From < proc_macro :: TokenStream > for TokenStream { fn from (inner : proc_macro :: TokenStream) -> Self { TokenStream :: from_str_unchecked (& inner . to_string ()) } }
    };
}

impl_94!();