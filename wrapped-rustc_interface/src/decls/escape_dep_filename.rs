macro_rules! escape_dep_filename {
    () => {
        fn escape_dep_filename (filename : & str) -> String { filename . replace (' ' , "\\ ") }
    };
}

escape_dep_filename!()