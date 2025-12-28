macro_rules! deps {
    () => {
        Segment!();
    };
}

macro_rules! PT_PHDR {
    () => {
        deps!();
        # [doc = " Segment contains the program header table."] pub const PT_PHDR : u32 = 6 ;
    };
}

PT_PHDR!()