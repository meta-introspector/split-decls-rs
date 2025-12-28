macro_rules! ARM64_RELOC_ADDEND {
    () => {
        # [doc = " must be followed by PAGE21 or PAGEOFF12"] pub const ARM64_RELOC_ADDEND : u8 = 10 ;
    };
}

ARM64_RELOC_ADDEND!();