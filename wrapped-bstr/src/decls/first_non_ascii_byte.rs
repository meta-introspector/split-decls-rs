macro_rules! first_non_ascii_byte {
    () => {
        # [doc = " Returns the index of the first non ASCII byte in the given slice."] # [doc = ""] # [doc = " If slice only contains ASCII bytes, then the length of the slice is"] # [doc = " returned."] pub fn first_non_ascii_byte (slice : & [u8]) -> usize { # [cfg (any (miri , not (target_arch = "x86_64")))] { first_non_ascii_byte_fallback (slice) } # [cfg (all (not (miri) , target_arch = "x86_64"))] { first_non_ascii_byte_sse2 (slice) } }
    };
}

first_non_ascii_byte!()