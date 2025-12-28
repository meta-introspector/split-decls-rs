macro_rules! deps {
    () => {
        Rela!();
    };
}

macro_rules! DT_RELASZ {
    () => {
        deps!();
        # [doc = " Total size of Rela relocs"] pub const DT_RELASZ : u32 = 8 ;
    };
}

DT_RELASZ!();