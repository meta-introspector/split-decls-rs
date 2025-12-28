macro_rules! completion_list_no_kw_with_private_editable {
    () => {
        pub (crate) fn completion_list_no_kw_with_private_editable (# [rust_analyzer :: rust_fixture] ra_fixture : & str ,) -> String { let mut config = TEST_CONFIG ; config . enable_private_editable = true ; completion_list_with_config (config , ra_fixture , false , None) }
    };
}

completion_list_no_kw_with_private_editable!();