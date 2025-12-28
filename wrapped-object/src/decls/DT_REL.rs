macro_rules! deps {
    () => {
        Rel!();
    };
}

macro_rules! DT_REL {
    () => {
        deps!();
        # [doc = " Address of Rel relocs"] pub const DT_REL : u32 = 17 ;
    };
}

DT_REL!()