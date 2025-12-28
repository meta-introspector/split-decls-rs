macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHT_GROUP {
    () => {
        deps!();
        # [doc = " Section group."] pub const SHT_GROUP : u32 = 17 ;
    };
}

SHT_GROUP!();