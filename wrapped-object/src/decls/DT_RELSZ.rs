macro_rules! deps {
    () => {
        Rel!();
    };
}

macro_rules! DT_RELSZ {
    () => {
        deps!();
        # [doc = " Total size of Rel relocs"] pub const DT_RELSZ : u32 = 18 ;
    };
}

DT_RELSZ!();