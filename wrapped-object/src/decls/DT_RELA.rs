macro_rules! deps {
    () => {
        Rela!();
    };
}

macro_rules! DT_RELA {
    () => {
        deps!();
        # [doc = " Address of Rela relocs"] pub const DT_RELA : u32 = 7 ;
    };
}

DT_RELA!();