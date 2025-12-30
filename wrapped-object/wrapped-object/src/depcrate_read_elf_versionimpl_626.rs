// Generated macro for impl_626 (impl)
macro_rules! Depcrate_read_elf_versionimpl_626 {
() => {
// Module: crate::read::elf::version
// Provides: {"impl_626"}
// Dependencies: {}
impl < 'data , Elf : FileHeader > Iterator for VerdefIterator < 'data , Elf > { type Item = Result < (& 'data elf :: Verdef < Elf :: Endian > , VerdauxIterator < 'data , Elf >) > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
};
}
