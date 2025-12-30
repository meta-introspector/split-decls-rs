// Generated macro for impl_597 (impl)
macro_rules! Depcrate_read_elf_noteimpl_597 {
() => {
// Module: crate::read::elf::note
// Provides: {"impl_597"}
// Dependencies: {}
impl < 'data , Endian : endian :: Endian > Iterator for GnuPropertyIterator < 'data , Endian > { type Item = read :: Result < GnuProperty < 'data > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
};
}
