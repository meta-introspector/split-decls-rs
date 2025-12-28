macro_rules! failed {
    () => {
        # [doc = " Tokens to be returned when the macro cannot proceed."] fn failed (crate_name : & Ident) -> proc_macro :: TokenStream { finish (quote ! { pub mod # crate_name { } } , quote ! { "" }) }
    };
}

failed!();