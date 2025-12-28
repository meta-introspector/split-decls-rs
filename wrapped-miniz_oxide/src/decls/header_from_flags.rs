macro_rules! header_from_flags {
    () => {
        # [doc = " Create a zlib header from the given compression flags."] # [doc = " Only level is considered."] # [inline] pub fn header_from_flags (flags : u32) -> [u8 ; 2] { let level = zlib_level_from_flags (flags) ; header_from_level (level , flags) }
    };
}

header_from_flags!();