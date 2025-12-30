// Generated macro for impl_655 (impl)
macro_rules! Depcrate_read_elf_attributesimpl_655 {
() => {
// Module: crate::read::elf::attributes
// Provides: {"impl_655"}
// Dependencies: {}
impl < 'data , Elf : FileHeader > Iterator for AttributesSubsubsectionIterator < 'data , Elf > { type Item = Result < AttributesSubsubsection < 'data > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
};
}
