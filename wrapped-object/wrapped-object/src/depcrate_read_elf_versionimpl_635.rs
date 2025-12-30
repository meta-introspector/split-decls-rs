// Generated macro for impl_635 (impl)
macro_rules! Depcrate_read_elf_versionimpl_635 {
() => {
// Module: crate::read::elf::version
// Provides: {"impl_635"}
// Dependencies: {}
impl < 'data , Elf : FileHeader > Iterator for VernauxIterator < 'data , Elf > { type Item = Result < & 'data elf :: Vernaux < Elf :: Endian > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
};
}
