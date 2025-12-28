macro_rules! deps {
    () => {
        Segment!();
    };
}

macro_rules! PF_X {
    () => {
        deps!();
        # [doc = " Segment is executable."] pub const PF_X : u32 = 1 << 0 ;
    };
}

PF_X!()