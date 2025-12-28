macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! STT_NOTYPE {
    () => {
        deps!();
        # [doc = " Symbol type is unspecified."] pub const STT_NOTYPE : u8 = 0 ;
    };
}

STT_NOTYPE!()