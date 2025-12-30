// Generated macro for Relocation (struct)
macro_rules! Depcrate_read_pe_relocationRelocation {
() => {
// Module: crate::read::pe::relocation
// Provides: {"Relocation"}
// Dependencies: {}
# [doc = " A relocation in the `.reloc` section of a PE file."] # [derive (Debug , Default , Clone , Copy)] pub struct Relocation { # [doc = " The virtual address of the relocation."] pub virtual_address : u32 , # [doc = " One of the `pe::IMAGE_REL_BASED_*` constants."] pub typ : u16 , }
};
}
