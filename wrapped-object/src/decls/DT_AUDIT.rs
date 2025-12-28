macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! DT_AUDIT {
    () => {
        deps!();
        # [doc = " Object auditing."] pub const DT_AUDIT : u32 = 0x6fff_fefc ;
    };
}

DT_AUDIT!()