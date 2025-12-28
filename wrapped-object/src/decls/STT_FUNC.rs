macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! STT_FUNC {
    () => {
        deps!();
        # [doc = " Symbol is a code object."] pub const STT_FUNC : u8 = 2 ;
    };
}

STT_FUNC!()