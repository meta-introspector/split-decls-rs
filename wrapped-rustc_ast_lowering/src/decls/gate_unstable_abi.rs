macro_rules! gate_unstable_abi {
    () => {
        # [allow (rustc :: untranslatable_diagnostic)] pub (crate) fn gate_unstable_abi (sess : & Session , features : & Features , span : Span , abi : ExternAbi) { match extern_abi_enabled (features , span , abi) { Ok (_) => () , Err (unstable_abi) => { let explain = unstable_abi . to_string () ; feature_err (sess , unstable_abi . feature , span , explain) . emit () ; } } }
    };
}

gate_unstable_abi!()