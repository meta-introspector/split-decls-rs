macro_rules! LZMA2_COMPRESSED_LIMIT {
    () => {
        const LZMA2_COMPRESSED_LIMIT : u32 = (64 << 10) - 26 ;
    };
}

LZMA2_COMPRESSED_LIMIT!()