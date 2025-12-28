macro_rules! first_non_ascii_byte_mask {
    () => {
        # [doc = " Compute the position of the first ASCII byte in the given mask."] # [doc = ""] # [doc = " The mask should be computed by `chunk & ASCII_MASK`, where `chunk` is"] # [doc = " 8 contiguous bytes of the slice being checked where *at least* one of those"] # [doc = " bytes is not an ASCII byte."] # [doc = ""] # [doc = " The position returned is always in the inclusive range [0, 7]."] # [cfg (any (test , miri , not (target_arch = "x86_64")))] fn first_non_ascii_byte_mask (mask : usize) -> usize { # [cfg (target_endian = "little")] { mask . trailing_zeros () as usize / 8 } # [cfg (target_endian = "big")] { mask . leading_zeros () as usize / 8 } }
    };
}

first_non_ascii_byte_mask!();