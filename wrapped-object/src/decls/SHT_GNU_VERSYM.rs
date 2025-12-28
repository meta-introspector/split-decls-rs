macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! SHT_GNU_VERSYM {
    () => {
        deps!();
        # [doc = " Version symbol table."] # [allow (non_upper_case_globals)] pub const SHT_GNU_VERSYM : u32 = 0x6fff_ffff ;
    };
}

SHT_GNU_VERSYM!()