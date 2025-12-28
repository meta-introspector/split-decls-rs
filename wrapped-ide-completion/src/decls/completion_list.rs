macro_rules! completion_list {
    () => {
        pub (crate) fn completion_list (# [rust_analyzer :: rust_fixture] ra_fixture : & str) -> String { completion_list_with_config (TEST_CONFIG , ra_fixture , true , None) }
    };
}

completion_list!();