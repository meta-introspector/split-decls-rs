macro_rules! header_from_level {
    () => {
        # [doc = " Get the zlib header for the level using the default window size and no"] # [doc = " dictionary."] # [inline] fn header_from_level (level : u8 , flags : u32) -> [u8 ; 2] { let cmf = cmf_from_flags (flags) ; [cmf , add_fcheck (cmf , level << 6)] }
    };
}

header_from_level!()