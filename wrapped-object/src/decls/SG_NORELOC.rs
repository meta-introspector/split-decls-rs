macro_rules! SG_NORELOC {
    () => {
        # [doc = " this segment has nothing that was relocated in it and nothing relocated to it, that is it maybe safely replaced without relocation"] pub const SG_NORELOC : u32 = 0x4 ;
    };
}

SG_NORELOC!();