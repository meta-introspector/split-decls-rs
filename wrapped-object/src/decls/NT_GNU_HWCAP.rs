macro_rules! NT_GNU_HWCAP {
    () => {
        # [doc = " Synthetic hwcap information."] # [doc = ""] # [doc = " The descriptor begins with two words:"] # [doc = " - word 0: number of entries"] # [doc = " - word 1: bitmask of enabled entries"] # [doc = ""] # [doc = " Then follow variable-length entries, one byte followed by a"] # [doc = " '\\0'-terminated hwcap name string.  The byte gives the bit"] # [doc = " number to test if enabled, (1U << bit) & bitmask.  */"] pub const NT_GNU_HWCAP : u32 = 2 ;
    };
}

NT_GNU_HWCAP!();