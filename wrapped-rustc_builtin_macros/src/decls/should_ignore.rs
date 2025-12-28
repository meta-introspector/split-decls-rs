macro_rules! should_ignore {
    () => {
        fn should_ignore (i : & ast :: Item) -> bool { attr :: contains_name (& i . attrs , sym :: ignore) }
    };
}

should_ignore!()