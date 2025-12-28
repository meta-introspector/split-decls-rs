macro_rules! ASCII_MASK_U64 {
    () => {
        # [cfg (any (test , miri , not (target_arch = "x86_64")))] const ASCII_MASK_U64 : u64 = 0x8080808080808080 ;
    };
}

ASCII_MASK_U64!();