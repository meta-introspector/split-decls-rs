// Generated macro for impl_1031 (impl)
macro_rules! Depcrate_read_pe_relocationimpl_1031 {
() => {
// Module: crate::read::pe::relocation
// Provides: {"impl_1031"}
// Dependencies: {}
impl < 'data > RelocationIterator < 'data > { # [doc = " Return the virtual address of the page that this block of relocations applies to."] pub fn virtual_address (& self) -> u32 { self . virtual_address } # [doc = " Return the size in bytes of this block of relocations."] pub fn size (& self) -> u32 { self . size } }
};
}
