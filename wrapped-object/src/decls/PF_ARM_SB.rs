macro_rules! deps {
    () => {
        Segment!();
    };
}

macro_rules! PF_ARM_SB {
    () => {
        deps!();
        # [doc = " Segment contains the location addressed by the static base."] pub const PF_ARM_SB : u32 = 0x1000_0000 ;
    };
}

PF_ARM_SB!()