macro_rules! is_blank {
    () => {
        fn is_blank (s : & str) -> bool { s . trim () . is_empty () }
    };
}

is_blank!()