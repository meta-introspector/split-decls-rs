macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! DT_MIPS_IVERSION {
    () => {
        deps!();
        # [doc = " Version string (string tbl index)"] pub const DT_MIPS_IVERSION : u32 = 0x7000_0004 ;
    };
}

DT_MIPS_IVERSION!()