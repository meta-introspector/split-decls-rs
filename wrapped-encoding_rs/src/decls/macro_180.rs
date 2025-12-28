macro_rules! macro_180 {
    () => {
        cfg_if ! { if # [cfg (target_endian = "little")] { # [allow (dead_code)] # [inline (always)] fn count_zeros (word : usize) -> u32 { word . trailing_zeros () } } else { # [allow (dead_code)] # [inline (always)] fn count_zeros (word : usize) -> u32 { word . leading_zeros () } } }
    };
}

macro_180!()