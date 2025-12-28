macro_rules! deps {
    () => {
        Table!();
    };
}

macro_rules! XMC_DB {
    () => {
        deps!();
        # [doc = " Debug Dictionary Table"] pub const XMC_DB : u8 = 2 ;
    };
}

XMC_DB!();