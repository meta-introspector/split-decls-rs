// Generated macro for impl_97 (impl)
macro_rules! Depcrate_deflate_coreimpl_97 {
() => {
// Module: crate::deflate::core
// Provides: {"impl_97"}
// Dependencies: {}
impl ParamsOxide { fn new (flags : u32) -> Self { ParamsOxide { flags , greedy_parsing : flags & TDEFL_GREEDY_PARSING_FLAG != 0 , block_index : 0 , saved_match_dist : 0 , saved_match_len : 0 , saved_lit : 0 , flush : TDEFLFlush :: None , flush_ofs : 0 , flush_remaining : 0 , finished : false , adler32 : MZ_ADLER32_INIT , src_pos : 0 , out_buf_ofs : 0 , prev_return_status : TDEFLStatus :: Okay , saved_bit_buffer : 0 , saved_bits_in : 0 , local_buf : Box :: default () , } } fn update_flags (& mut self , flags : u32) { self . flags = flags ; self . greedy_parsing = self . flags & TDEFL_GREEDY_PARSING_FLAG != 0 ; } # [doc = " Reset state, saving settings."] fn reset (& mut self) { self . block_index = 0 ; self . saved_match_len = 0 ; self . saved_match_dist = 0 ; self . saved_lit = 0 ; self . flush = TDEFLFlush :: None ; self . flush_ofs = 0 ; self . flush_remaining = 0 ; self . finished = false ; self . adler32 = MZ_ADLER32_INIT ; self . src_pos = 0 ; self . out_buf_ofs = 0 ; self . prev_return_status = TDEFLStatus :: Okay ; self . saved_bit_buffer = 0 ; self . saved_bits_in = 0 ; self . local_buf . b = [0 ; OUT_BUF_SIZE] ; } }
};
}
