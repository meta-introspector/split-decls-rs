macro_rules! S_EMPTY_ERROR {
    () => {
        # [doc = " We remap S_OK to this error because the S_OK representation (zero) is reserved for niche"] # [doc = " optimizations."] const S_EMPTY_ERROR : NonZeroI32 = const_nonzero_i32 (u32 :: from_be_bytes (* b"S_OK") as i32) ;
    };
}

S_EMPTY_ERROR!()