macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! VERSYM_HIDDEN {
    () => {
        deps!();
        # [doc = " Symbol is hidden."] pub const VERSYM_HIDDEN : u16 = 0x8000 ;
    };
}

VERSYM_HIDDEN!();