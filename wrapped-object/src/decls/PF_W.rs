macro_rules! deps {
    () => {
        Segment!();
    };
}

macro_rules! PF_W {
    () => {
        deps!();
        # [doc = " Segment is writable."] pub const PF_W : u32 = 1 << 1 ;
    };
}

PF_W!()