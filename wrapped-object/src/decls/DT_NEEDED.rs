macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! DT_NEEDED {
    () => {
        deps!();
        # [doc = " Name of needed library"] pub const DT_NEEDED : u32 = 1 ;
    };
}

DT_NEEDED!();