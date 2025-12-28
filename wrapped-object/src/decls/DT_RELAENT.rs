macro_rules! deps {
    () => {
        Rela!();
    };
}

macro_rules! DT_RELAENT {
    () => {
        deps!();
        # [doc = " Size of one Rela reloc"] pub const DT_RELAENT : u32 = 9 ;
    };
}

DT_RELAENT!();