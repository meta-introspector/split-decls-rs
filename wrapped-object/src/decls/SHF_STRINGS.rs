macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! SHF_STRINGS {
    () => {
        deps!();
        # [doc = " Section contains nul-terminated strings."] pub const SHF_STRINGS : u32 = 1 << 5 ;
    };
}

SHF_STRINGS!()