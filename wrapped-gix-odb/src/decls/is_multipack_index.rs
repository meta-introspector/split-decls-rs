macro_rules! is_multipack_index {
    () => {
        fn is_multipack_index (path : & Path) -> bool { path . file_name () == Some (OsStr :: new ("multi-pack-index")) }
    };
}

is_multipack_index!()