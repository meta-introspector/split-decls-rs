macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHF_GROUP {
    () => {
        deps!();
        # [doc = " Section is a member of a group."] pub const SHF_GROUP : u32 = 1 << 9 ;
    };
}

SHF_GROUP!();