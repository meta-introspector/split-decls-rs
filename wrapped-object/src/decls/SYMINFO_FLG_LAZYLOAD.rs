macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! SYMINFO_FLG_LAZYLOAD {
    () => {
        deps!();
        # [doc = " Symbol bound to object to be lazy loaded"] pub const SYMINFO_FLG_LAZYLOAD : u16 = 0x0008 ;
    };
}

SYMINFO_FLG_LAZYLOAD!();