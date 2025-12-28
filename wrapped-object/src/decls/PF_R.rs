macro_rules! deps {
    () => {
        Segment!();
    };
}

macro_rules! PF_R {
    () => {
        deps!();
        # [doc = " Segment is readable."] pub const PF_R : u32 = 1 << 2 ;
    };
}

PF_R!();