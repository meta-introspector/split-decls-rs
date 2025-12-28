macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! SHT_GNU_VERNEED {
    () => {
        deps!();
        # [doc = " Version needs section."] # [allow (non_upper_case_globals)] pub const SHT_GNU_VERNEED : u32 = 0x6fff_fffe ;
    };
}

SHT_GNU_VERNEED!();