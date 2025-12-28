macro_rules! TreeEntry {
    () => {
        pub (crate) struct TreeEntry { pub id : gix_hash :: ObjectId , pub crc32 : u32 , }
    };
}

TreeEntry!()