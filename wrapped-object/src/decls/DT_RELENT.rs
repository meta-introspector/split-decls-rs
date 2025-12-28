macro_rules! deps {
    () => {
        Rel!();
    };
}

macro_rules! DT_RELENT {
    () => {
        deps!();
        # [doc = " Size of one Rel reloc"] pub const DT_RELENT : u32 = 19 ;
    };
}

DT_RELENT!()