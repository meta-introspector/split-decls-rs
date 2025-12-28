macro_rules! non_empty {
    () => {
        fn non_empty (var : Option < & std :: ffi :: OsStr >) -> bool { ! var . unwrap_or_default () . is_empty () }
    };
}

non_empty!()