macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! SHT_GNU_VERDEF {
    () => {
        deps!();
        # [doc = " Version definition section."] # [allow (non_upper_case_globals)] pub const SHT_GNU_VERDEF : u32 = 0x6fff_fffd ;
    };
}

SHT_GNU_VERDEF!()