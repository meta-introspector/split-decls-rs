macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! STT_GNU_IFUNC {
    () => {
        deps!();
        # [doc = " Symbol is an indirect code object."] pub const STT_GNU_IFUNC : u8 = 10 ;
    };
}

STT_GNU_IFUNC!()