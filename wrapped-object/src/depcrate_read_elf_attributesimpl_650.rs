// Generated macro for impl_650 (impl)
macro_rules! Depcrate_read_elf_attributesimpl_650 {
() => {
// Module: crate::read::elf::attributes
// Provides: {"impl_650"}
// Dependencies: {}
impl < 'data , Elf : FileHeader > Iterator for AttributesSubsectionIterator < 'data , Elf > { type Item = Result < AttributesSubsection < 'data , Elf > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
};
}
