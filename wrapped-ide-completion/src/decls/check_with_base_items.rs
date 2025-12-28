macro_rules! check_with_base_items {
    () => {
        pub (crate) fn check_with_base_items (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect ,) { check (& format ! ("{BASE_ITEMS_FIXTURE}{ra_fixture}") , expect) }
    };
}

check_with_base_items!();