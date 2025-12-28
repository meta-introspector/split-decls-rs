macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        # [cfg (feature = "proc-macro")] impl From < TokenStream > for proc_macro :: TokenStream { fn from (inner : TokenStream) -> Self { proc_macro :: TokenStream :: from_str_unchecked (& inner . to_string ()) } }
    };
}

impl_95!();