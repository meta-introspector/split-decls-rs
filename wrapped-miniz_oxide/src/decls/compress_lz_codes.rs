macro_rules! deps {
    () => {
        OutputBufferOxide!();
        HuffmanOxide!();
        Result!();
        BitBuffer!();
    };
}

macro_rules! compress_lz_codes {
    () => {
        deps!();
        fn compress_lz_codes (huff : & HuffmanOxide , output : & mut OutputBufferOxide , lz_code_buf : & [u8 ; LZ_CODE_BUF_SIZE] , lz_code_buf_used_len : usize ,) -> Result < bool > { let mut flags = 1 ; let mut bb = BitBuffer { bit_buffer : u64 :: from (output . bit_buffer) , bits_in : output . bits_in , } ; let lz_code_buf_used_len = cmp :: min (lz_code_buf . len () , lz_code_buf_used_len) ; let mut i : usize = 0 ; while i < lz_code_buf_used_len { if flags == 1 { flags = u32 :: from (lz_code_buf [i]) | 0x100 ; i += 1 ; } if flags & 1 == 1 { flags >>= 1 ; let sym ; let num_extra_bits ; let match_len = lz_code_buf [i & LZ_CODE_BUF_MASK] as usize ; let match_dist = lz_code_buf [(i + 1) & LZ_CODE_BUF_MASK] as u16 | ((lz_code_buf [(i + 2) & LZ_CODE_BUF_MASK] as u16) << 8) ; i += 3 ; debug_assert ! (huff . code_sizes [0] [LEN_SYM [match_len] as usize + LEN_SYM_OFFSET] != 0) ; let len_sym = (LEN_SYM [match_len] & 31) as usize + LEN_SYM_OFFSET ; bb . put_fast (u64 :: from (huff . codes [0] [len_sym]) , u32 :: from (huff . code_sizes [0] [len_sym]) ,) ; bb . put_fast (match_len as u64 & u64 :: from (BITMASKS [(LEN_EXTRA [match_len] & 7) as usize]) , u32 :: from (LEN_EXTRA [match_len]) ,) ; if match_dist < 512 { sym = SMALL_DIST_SYM [match_dist as usize] as usize ; num_extra_bits = SMALL_DIST_EXTRA [match_dist as usize] as usize ; } else { sym = LARGE_DIST_SYM [(match_dist >> 8) as usize] as usize ; num_extra_bits = LARGE_DIST_EXTRA [(match_dist >> 8) as usize] as usize ; } debug_assert ! (huff . code_sizes [1] [sym] != 0) ; bb . put_fast (u64 :: from (huff . codes [1] [sym]) , u32 :: from (huff . code_sizes [1] [sym]) ,) ; bb . put_fast (u64 :: from (match_dist) & u64 :: from (BITMASKS [num_extra_bits & 15]) , num_extra_bits as u32 ,) ; } else { for _ in 0 .. 3 { flags >>= 1 ; let lit = lz_code_buf [i & LZ_CODE_BUF_MASK] ; i += 1 ; debug_assert ! (huff . code_sizes [0] [lit as usize] != 0) ; bb . put_fast (u64 :: from (huff . codes [0] [lit as usize]) , u32 :: from (huff . code_sizes [0] [lit as usize]) ,) ; if flags & 1 == 1 || i >= lz_code_buf_used_len { break ; } } } bb . flush (output) ? ; } output . bits_in = 0 ; output . bit_buffer = 0 ; while bb . bits_in != 0 { let n = cmp :: min (bb . bits_in , 16) ; output . put_bits (bb . bit_buffer as u32 & BITMASKS [n as usize] , n) ; bb . bit_buffer >>= n ; bb . bits_in -= n ; } output . put_bits (u32 :: from (huff . codes [0] [256]) , u32 :: from (huff . code_sizes [0] [256]) ,) ; Ok (true) }
    };
}

compress_lz_codes!();