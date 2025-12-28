macro_rules! deps {
    () => {
        WideChar!();
    };
}

macro_rules! analyze_source_file_neon {
    () => {
        deps!();
        # [target_feature (enable = "neon")] # [cfg (all (target_arch = "aarch64" , target_endian = "little"))] # [allow (unsafe_op_in_unsafe_fn)] unsafe fn analyze_source_file_neon (src : & str , lines : & mut Vec < TextSize > , multi_byte_chars : & mut IntMap < u32 , Vec < WideChar > > ,) { use std :: arch :: aarch64 :: * ; const CHUNK_SIZE : usize = 16 ; let src_bytes = src . as_bytes () ; let chunk_count = src . len () / CHUNK_SIZE ; let newline = vdupq_n_s8 (b'\n' as i8) ; let mut intra_chunk_offset = 0 ; for chunk_index in 0 .. chunk_count { let ptr = src_bytes . as_ptr () as * const i8 ; let chunk = unsafe { vld1q_s8 (ptr . add (chunk_index * CHUNK_SIZE)) } ; let multibyte_test = vcltzq_s8 (chunk) ; let multibyte_mask = unsafe { move_mask (multibyte_test) } ; if multibyte_mask == 0 { assert ! (intra_chunk_offset == 0) ; let newlines_test = vceqq_s8 (chunk , newline) ; let mut newlines_mask = unsafe { move_mask (newlines_test) } ; if newlines_mask != 0 { let output_offset = TextSize :: from ((chunk_index * CHUNK_SIZE + 1) as u32) ; while newlines_mask != 0 { let trailing_zeros = newlines_mask . trailing_zeros () ; let index = trailing_zeros / 4 ; lines . push (TextSize :: from (index) + output_offset) ; newlines_mask &= (! 0xF) << trailing_zeros ; } } continue ; } let scan_start = chunk_index * CHUNK_SIZE + intra_chunk_offset ; intra_chunk_offset = analyze_source_file_generic (& src [scan_start ..] , CHUNK_SIZE - intra_chunk_offset , TextSize :: from (scan_start as u32) , lines , multi_byte_chars ,) ; } let tail_start = chunk_count * CHUNK_SIZE + intra_chunk_offset ; if tail_start < src . len () { analyze_source_file_generic (& src [tail_start ..] , src . len () - tail_start , TextSize :: from (tail_start as u32) , lines , multi_byte_chars ,) ; } }
    };
}

analyze_source_file_neon!()