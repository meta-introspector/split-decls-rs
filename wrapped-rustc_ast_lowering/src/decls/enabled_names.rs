macro_rules! enabled_names {
    () => {
        pub (crate) fn enabled_names (features : & rustc_feature :: Features , span : Span) -> Vec < & 'static str > { ExternAbi :: ALL_VARIANTS . into_iter () . filter (| abi | extern_abi_enabled (features , span , * * abi) . is_ok ()) . map (| abi | abi . as_str ()) . collect () }
    };
}

enabled_names!()