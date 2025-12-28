macro_rules! macro_17 {
    () => {
        document_select_macro ! { # [cfg (feature = "std")] # [macro_export] macro_rules ! select { ($ ($ tokens : tt) *) => { { use $ crate :: __private as __futures_crate ; $ crate :: select_internal ! { $ ($ tokens) * } } } } # [macro_export] macro_rules ! select_biased { ($ ($ tokens : tt) *) => { { use $ crate :: __private as __futures_crate ; $ crate :: select_biased_internal ! { $ ($ tokens) * } } } } }
    };
}

macro_17!()