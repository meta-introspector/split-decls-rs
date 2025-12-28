macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! STT_COMMON {
    () => {
        deps!();
        # [doc = " Symbol is a common data object."] pub const STT_COMMON : u8 = 5 ;
    };
}

STT_COMMON!();