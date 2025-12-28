macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! DT_SONAME {
    () => {
        deps!();
        # [doc = " Name of shared object"] pub const DT_SONAME : u32 = 14 ;
    };
}

DT_SONAME!()