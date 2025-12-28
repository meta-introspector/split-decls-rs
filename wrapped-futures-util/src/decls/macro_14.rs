macro_rules! macro_14 {
    () => {
        document_join_macro ! { # [macro_export] macro_rules ! join { ($ ($ tokens : tt) *) => { { use $ crate :: __private as __futures_crate ; $ crate :: join_internal ! { $ ($ tokens) * } } } } # [macro_export] macro_rules ! try_join { ($ ($ tokens : tt) *) => { { use $ crate :: __private as __futures_crate ; $ crate :: try_join_internal ! { $ ($ tokens) * } } } } }
    };
}

macro_14!()