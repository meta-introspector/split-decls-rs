macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHF_OS_NONCONFORMING {
    () => {
        deps!();
        # [doc = " Section requires special OS-specific handling."] pub const SHF_OS_NONCONFORMING : u32 = 1 << 8 ;
    };
}

SHF_OS_NONCONFORMING!();