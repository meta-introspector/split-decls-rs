macro_rules! is_dylib {
    () => {
        fn is_dylib (path : & Utf8Path) -> bool { match path . extension () . map (| e | e . to_owned () . to_lowercase ()) { None => false , Some (ext) => matches ! (ext . as_str () , "dll" | "dylib" | "so") , } }
    };
}

is_dylib!()