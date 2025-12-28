macro_rules! LZ_HASH_SIZE {
    () => {
        # [doc = " Size of the chained hash tables."] pub const LZ_HASH_SIZE : usize = 1 << LZ_HASH_BITS ;
    };
}

LZ_HASH_SIZE!();