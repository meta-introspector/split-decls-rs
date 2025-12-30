// Generated macro for PPC_RELOC_HA16 (const)
macro_rules! Depcrate_machoPPC_RELOC_HA16 {
() => {
// Module: crate::macho
// Provides: {"PPC_RELOC_HA16"}
// Dependencies: {}
# [doc = " Same as the RELOC_HI16 except the low 16 bits and the high 16 bits are added together"] # [doc = " with the low 16 bits sign extended first.  This means if bit 15 of the low 16 bits is"] # [doc = " set the high 16 bits stored in the instruction will be adjusted."] pub const PPC_RELOC_HA16 : u8 = 6 ;
};
}
