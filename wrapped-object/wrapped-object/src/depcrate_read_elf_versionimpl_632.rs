// Generated macro for impl_632 (impl)
macro_rules! Depcrate_read_elf_versionimpl_632 {
() => {
// Module: crate::read::elf::version
// Provides: {"impl_632"}
// Dependencies: {}
impl < 'data , Elf : FileHeader > Iterator for VerneedIterator < 'data , Elf > { type Item = Result < (& 'data elf :: Verneed < Elf :: Endian > , VernauxIterator < 'data , Elf > ,) > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
};
}
