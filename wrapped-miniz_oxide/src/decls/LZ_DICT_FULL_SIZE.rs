macro_rules! LZ_DICT_FULL_SIZE {
    () => {
        pub const LZ_DICT_FULL_SIZE : usize = LZ_DICT_SIZE + MAX_MATCH_LEN - 1 + 1 ;
    };
}

LZ_DICT_FULL_SIZE!();