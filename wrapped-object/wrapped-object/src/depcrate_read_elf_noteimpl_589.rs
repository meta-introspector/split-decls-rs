// Generated macro for impl_589 (impl)
macro_rules! Depcrate_read_elf_noteimpl_589 {
() => {
// Module: crate::read::elf::note
// Provides: {"impl_589"}
// Dependencies: {}
impl < 'data , Elf : FileHeader > Iterator for NoteIterator < 'data , Elf > { type Item = read :: Result < Note < 'data , Elf > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
};
}
