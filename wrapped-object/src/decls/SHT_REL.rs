macro_rules! deps {
    () => {
        Relocation!();
    };
}

macro_rules! SHT_REL {
    () => {
        deps!();
        # [doc = " Relocation entries without explicit addends."] pub const SHT_REL : u32 = 9 ;
    };
}

SHT_REL!();