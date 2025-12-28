macro_rules! empty_path {
    () => {
        fn empty_path () -> BString { BString :: default () }
    };
}

empty_path!()