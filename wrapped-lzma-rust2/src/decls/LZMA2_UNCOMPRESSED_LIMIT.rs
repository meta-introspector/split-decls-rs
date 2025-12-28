macro_rules! LZMA2_UNCOMPRESSED_LIMIT {
    () => {
        const LZMA2_UNCOMPRESSED_LIMIT : u32 = (2 << 20) - MATCH_LEN_MAX as u32 ;
    };
}

LZMA2_UNCOMPRESSED_LIMIT!();