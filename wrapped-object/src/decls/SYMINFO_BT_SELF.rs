macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! SYMINFO_BT_SELF {
    () => {
        deps!();
        # [doc = " Symbol bound to self"] pub const SYMINFO_BT_SELF : u16 = 0xffff ;
    };
}

SYMINFO_BT_SELF!()