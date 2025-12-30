// Generated macro for impl_1032 (impl)
macro_rules! Depcrate_read_pe_relocationimpl_1032 {
() => {
// Module: crate::read::pe::relocation
// Provides: {"impl_1032"}
// Dependencies: {}
impl < 'data > Iterator for RelocationIterator < 'data > { type Item = Relocation ; fn next (& mut self) -> Option < Relocation > { loop { let reloc = self . relocs . next () ? . get (LE) ; if reloc != 0 { return Some (Relocation { virtual_address : self . virtual_address . wrapping_add ((reloc & 0xfff) as u32) , typ : reloc >> 12 , }) ; } } } }
};
}
