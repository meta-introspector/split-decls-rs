// Generated macro for impl_660 (impl)
macro_rules! Depcrate_read_elf_attributesimpl_660 {
() => {
// Module: crate::read::elf::attributes
// Provides: {"impl_660"}
// Dependencies: {}
impl < 'data > Iterator for AttributeIndexIterator < 'data > { type Item = Result < u32 > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
};
}
