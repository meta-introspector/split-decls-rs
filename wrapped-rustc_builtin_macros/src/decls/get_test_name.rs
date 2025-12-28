macro_rules! get_test_name {
    () => {
        fn get_test_name (i : & ast :: Item) -> Option < Symbol > { attr :: first_attr_value_str_by_name (& i . attrs , sym :: rustc_test_marker) }
    };
}

get_test_name!()