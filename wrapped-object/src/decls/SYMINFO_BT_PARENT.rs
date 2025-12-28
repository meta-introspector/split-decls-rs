macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! SYMINFO_BT_PARENT {
    () => {
        deps!();
        # [doc = " Symbol bound to parent"] pub const SYMINFO_BT_PARENT : u16 = 0xfffe ;
    };
}

SYMINFO_BT_PARENT!()