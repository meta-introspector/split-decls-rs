macro_rules! LZ_HASH_SHIFT {
    () => {
        # [doc = " How many bits to shift when updating the current hash value."] pub const LZ_HASH_SHIFT : i32 = (LZ_HASH_BITS + 2) / 3 ;
    };
}

LZ_HASH_SHIFT!()