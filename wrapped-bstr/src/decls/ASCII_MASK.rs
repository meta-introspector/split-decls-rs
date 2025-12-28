macro_rules! ASCII_MASK {
    () => {
        # [cfg (any (test , miri , not (target_arch = "x86_64")))] const ASCII_MASK : usize = ASCII_MASK_U64 as usize ;
    };
}

ASCII_MASK!();