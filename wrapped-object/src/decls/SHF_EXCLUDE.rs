macro_rules! SHF_EXCLUDE {
    () => {
        # [doc = " This section is excluded from the final executable or shared library."] pub const SHF_EXCLUDE : u32 = 0x8000_0000 ;
    };
}

SHF_EXCLUDE!()