// Generated macro for PN_XNUM (const)
macro_rules! Depcrate_elfPN_XNUM {
() => {
// Module: crate::elf
// Provides: {"PN_XNUM"}
// Dependencies: {}
# [doc = " Special value for `FileHeader*::e_phnum`."] # [doc = ""] # [doc = " This indicates that the real number of program headers is too large to fit into e_phnum."] # [doc = " Instead the real value is in the field `sh_info` of section 0."] pub const PN_XNUM : u16 = 0xffff ;
};
}
