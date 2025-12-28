macro_rules! deps {
    () => {
        Relocation!();
    };
}

macro_rules! SHT_RELR {
    () => {
        deps!();
        # [doc = " Relocation entries; only offsets."] pub const SHT_RELR : u32 = 19 ;
    };
}

SHT_RELR!();