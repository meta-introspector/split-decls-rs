macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! SHT_GNU_ATTRIBUTES {
    () => {
        deps!();
        # [doc = " Object attributes."] pub const SHT_GNU_ATTRIBUTES : u32 = 0x6fff_fff5 ;
    };
}

SHT_GNU_ATTRIBUTES!();