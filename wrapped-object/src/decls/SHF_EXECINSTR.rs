macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHF_EXECINSTR {
    () => {
        deps!();
        # [doc = " Section is executable."] pub const SHF_EXECINSTR : u32 = 1 << 2 ;
    };
}

SHF_EXECINSTR!()