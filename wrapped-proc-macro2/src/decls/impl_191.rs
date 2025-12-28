macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        # [cfg (feature = "proc-macro")] # [cfg_attr (docsrs , doc (cfg (feature = "proc-macro")))] impl From < TokenStream > for proc_macro :: TokenStream { fn from (inner : TokenStream) -> Self { proc_macro :: TokenStream :: from (inner . inner) } }
    };
}

impl_191!()