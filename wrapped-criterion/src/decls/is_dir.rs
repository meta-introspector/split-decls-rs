macro_rules! is_dir {
    () => {
        pub fn is_dir < P > (path : & P) -> bool where P : AsRef < Path > , { let path : & Path = path . as_ref () ; path . is_dir () }
    };
}

is_dir!();