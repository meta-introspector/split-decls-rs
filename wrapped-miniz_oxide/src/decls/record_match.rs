macro_rules! deps {
    () => {
        HuffmanOxide!();
        LZOxide!();
    };
}

macro_rules! record_match {
    () => {
        deps!();
        fn record_match (h : & mut HuffmanOxide , lz : & mut LZOxide , match_len : u32 , mut match_dist : u32) { debug_assert ! (match_len >= MIN_MATCH_LEN . into ()) ; debug_assert ! (match_dist >= 1) ; debug_assert ! (match_dist as usize <= LZ_DICT_SIZE) ; lz . total_bytes += match_len ; match_dist -= 1 ; let match_len = (match_len - u32 :: from (MIN_MATCH_LEN)) as u8 ; lz . write_code (match_len) ; lz . write_code (match_dist as u8) ; lz . write_code ((match_dist >> 8) as u8) ; * lz . get_flag () >>= 1 ; * lz . get_flag () |= 0x80 ; lz . consume_flag () ; let symbol = if match_dist < 512 { SMALL_DIST_SYM [match_dist as usize] } else { LARGE_DIST_SYM [((match_dist >> 8) & 127) as usize] } as usize ; h . count [1] [symbol] += 1 ; h . count [0] [(LEN_SYM [match_len as usize] as usize & 31) + LEN_SYM_OFFSET] += 1 ; }
    };
}

record_match!()