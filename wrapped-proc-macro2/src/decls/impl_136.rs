macro_rules! deps {
    () => {
        TokenStream!();
        FromStr2!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        # [cfg (feature = "proc-macro")] impl FromStr2 for proc_macro :: TokenStream { # [cfg (wrap_proc_macro)] fn valid (src : & str) -> bool { TokenStream :: from_str_checked (src) . is_ok () } }
    };
}

impl_136!()