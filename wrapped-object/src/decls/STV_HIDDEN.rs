macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! STV_HIDDEN {
    () => {
        deps!();
        # [doc = " Symbol is not visible to other components."] pub const STV_HIDDEN : u8 = 2 ;
    };
}

STV_HIDDEN!();