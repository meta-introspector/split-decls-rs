macro_rules! Block {
    () => {
        # [derive (Debug , Clone)] struct Block { start_pos : u64 , unpadded_size : u64 , uncompressed_size : u64 , }
    };
}

Block!();