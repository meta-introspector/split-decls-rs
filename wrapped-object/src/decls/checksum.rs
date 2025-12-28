macro_rules! checksum {
    () => {
        fn checksum (data : & [u8]) -> u32 { let mut hasher = crc32fast :: Hasher :: new_with_initial (0xffff_ffff) ; hasher . update (data) ; ! hasher . finalize () }
    };
}

checksum!()