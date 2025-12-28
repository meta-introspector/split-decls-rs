macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! DF_SYMBOLIC {
    () => {
        deps!();
        # [doc = " Symbol resolutions starts here"] pub const DF_SYMBOLIC : u32 = 0x0000_0002 ;
    };
}

DF_SYMBOLIC!()