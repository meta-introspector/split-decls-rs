macro_rules! combine_words {
    () => {
        # [inline] fn combine_words (hi : u32 , lo : u32) -> f64 { f64 :: from_bits (((hi as u64) << 32) | lo as u64) }
    };
}

combine_words!()