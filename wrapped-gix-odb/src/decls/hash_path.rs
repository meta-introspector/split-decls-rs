macro_rules! hash_path {
    () => {
        fn hash_path (id : & gix_hash :: oid , mut root : PathBuf) -> PathBuf { let mut hex = gix_hash :: Kind :: hex_buf () ; let hex = id . hex_to_buf (hex . as_mut ()) ; root . push (& hex [.. 2]) ; root . push (& hex [2 ..]) ; root }
    };
}

hash_path!();