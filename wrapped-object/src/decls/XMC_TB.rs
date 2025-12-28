macro_rules! deps {
    () => {
        Table!();
    };
}

macro_rules! XMC_TB {
    () => {
        deps!();
        # [doc = " Traceback Table csect"] pub const XMC_TB : u8 = 13 ;
    };
}

XMC_TB!()