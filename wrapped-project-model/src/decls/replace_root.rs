macro_rules! replace_root {
    () => {
        fn replace_root (s : & mut String , direction : bool) { if direction { let root = if cfg ! (windows) { r#"C:\\ROOT\"# } else { "/ROOT/" } ; * s = s . replace ("$ROOT$" , root) } else { let root = if cfg ! (windows) { r#"C:\\\\ROOT\\"# } else { "/ROOT/" } ; * s = s . replace (root , "$ROOT$") } }
    };
}

replace_root!()