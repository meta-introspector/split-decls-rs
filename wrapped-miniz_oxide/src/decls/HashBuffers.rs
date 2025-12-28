macro_rules! HashBuffers {
    () => {
        pub struct HashBuffers { pub dict : Box < [u8 ; LZ_DICT_FULL_SIZE] > , pub next : Box < [u16 ; LZ_DICT_SIZE] > , pub hash : Box < [u16 ; LZ_DICT_SIZE] > , }
    };
}

HashBuffers!()