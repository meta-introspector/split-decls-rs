macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! STT_OBJECT {
    () => {
        deps!();
        # [doc = " Symbol is a data object."] pub const STT_OBJECT : u8 = 1 ;
    };
}

STT_OBJECT!()