macro_rules! deps {
    () => {
        Relocation!();
    };
}

macro_rules! SHT_RELA {
    () => {
        deps!();
        # [doc = " Relocation entries with explicit addends."] pub const SHT_RELA : u32 = 4 ;
    };
}

SHT_RELA!()