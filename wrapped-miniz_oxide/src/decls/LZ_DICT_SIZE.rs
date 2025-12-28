macro_rules! LZ_DICT_SIZE {
    () => {
        # [doc = " Size of the chained hash table."] pub (crate) const LZ_DICT_SIZE : usize = 32_768 ;
    };
}

LZ_DICT_SIZE!()