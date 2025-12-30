// Generated macro for impl_629 (impl)
macro_rules! Depcrate_read_elf_versionimpl_629 {
() => {
// Module: crate::read::elf::version
// Provides: {"impl_629"}
// Dependencies: {}
impl < 'data , Elf : FileHeader > Iterator for VerdauxIterator < 'data , Elf > { type Item = Result < & 'data elf :: Verdaux < Elf :: Endian > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
};
}
