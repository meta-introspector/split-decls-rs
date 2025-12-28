macro_rules! is_valid_version {
    () => {
        fn is_valid_version (word : & str) -> bool { ! word . starts_with ('.') && ! word . ends_with ('.') }
    };
}

is_valid_version!()