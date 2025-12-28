macro_rules! deps {
    () => {
        CVWords!();
        Platform!();
    };
}

macro_rules! ChunkState {
    () => {
        deps!();
        # [derive (Clone)] struct ChunkState { cv : CVWords , chunk_counter : u64 , buf : [u8 ; BLOCK_LEN] , buf_len : u8 , blocks_compressed : u8 , flags : u8 , platform : Platform , }
    };
}

ChunkState!();