macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHF_ALLOC {
    () => {
        deps!();
        # [doc = " Section occupies memory during execution."] pub const SHF_ALLOC : u32 = 1 << 1 ;
    };
}

SHF_ALLOC!();