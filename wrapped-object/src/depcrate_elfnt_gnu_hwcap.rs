// Generated macro for NT_GNU_HWCAP (const)
macro_rules! Depcrate_elfNT_GNU_HWCAP {
() => {
// Module: crate::elf
// Provides: {"NT_GNU_HWCAP"}
// Dependencies: {}
# [doc = " Synthetic hwcap information."] # [doc = ""] # [doc = " The descriptor begins with two words:"] # [doc = " - word 0: number of entries"] # [doc = " - word 1: bitmask of enabled entries"] # [doc = ""] # [doc = " Then follow variable-length entries, one byte followed by a"] # [doc = " '\\0'-terminated hwcap name string.  The byte gives the bit"] # [doc = " number to test if enabled, (1U << bit) & bitmask.  */"] pub const NT_GNU_HWCAP : u32 = 2 ;
};
}
