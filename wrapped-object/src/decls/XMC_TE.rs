macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! XMC_TE {
    () => {
        deps!();
        # [doc = " Symbol mapped at the end of TOC"] pub const XMC_TE : u8 = 22 ;
    };
}

XMC_TE!()