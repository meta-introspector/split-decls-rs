macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! DF_TEXTREL {
    () => {
        deps!();
        # [doc = " Object contains text relocations"] pub const DF_TEXTREL : u32 = 0x0000_0004 ;
    };
}

DF_TEXTREL!()