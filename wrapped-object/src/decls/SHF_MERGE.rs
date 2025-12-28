macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHF_MERGE {
    () => {
        deps!();
        # [doc = " Section may be be merged to eliminate duplication."] pub const SHF_MERGE : u32 = 1 << 4 ;
    };
}

SHF_MERGE!();