macro_rules! get_test_path {
    () => {
        fn get_test_path (file : & str) -> Utf8PathBuf { let base = Utf8PathBuf :: from (env ! ("CARGO_MANIFEST_DIR")) ; base . join ("test_data") . join (file) }
    };
}

get_test_path!();