macro_rules! involves_submodule {
    () => {
        fn involves_submodule (a : & EntryMode , b : & EntryMode) -> bool { a . is_commit () || b . is_commit () }
    };
}

involves_submodule!();