macro_rules! deps {
    () => {
        TDEFLStatus!();
        TDEFLFlush!();
        LocalBuf!();
    };
}

macro_rules! ParamsOxide {
    () => {
        deps!();
        pub (crate) struct ParamsOxide { pub flags : u32 , pub greedy_parsing : bool , pub block_index : u32 , pub saved_match_dist : u32 , pub saved_match_len : u32 , pub saved_lit : u8 , pub flush : TDEFLFlush , pub flush_ofs : u32 , pub flush_remaining : u32 , pub finished : bool , pub adler32 : u32 , pub src_pos : usize , pub out_buf_ofs : usize , pub prev_return_status : TDEFLStatus , pub saved_bit_buffer : u32 , pub saved_bits_in : u32 , pub local_buf : Box < LocalBuf > , }
    };
}

ParamsOxide!()