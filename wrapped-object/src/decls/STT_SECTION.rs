macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! STT_SECTION {
    () => {
        deps!();
        # [doc = " Symbol is associated with a section."] pub const STT_SECTION : u8 = 3 ;
    };
}

STT_SECTION!();