macro_rules! path2url {
    () => {
        pub fn path2url (path : & Path) -> String { Url :: from_file_path (path) . unwrap () . to_string () }
    };
}

path2url!();