macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        # [cfg (feature = "proc-macro")] # [cfg_attr (docsrs , doc (cfg (feature = "proc-macro")))] impl From < proc_macro :: TokenStream > for TokenStream { fn from (inner : proc_macro :: TokenStream) -> Self { TokenStream :: _new (imp :: TokenStream :: from (inner)) } }
    };
}

impl_190!();