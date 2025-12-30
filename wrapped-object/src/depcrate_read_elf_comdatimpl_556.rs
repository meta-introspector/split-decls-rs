// Generated macro for impl_556 (impl)
macro_rules! Depcrate_read_elf_comdatimpl_556 {
() => {
// Module: crate::read::elf::comdat
// Provides: {"impl_556"}
// Dependencies: {}
impl < 'data , 'file , Elf , R > Iterator for ElfComdatSectionIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { type Item = SectionIndex ; fn next (& mut self) -> Option < Self :: Item > { let index = self . sections . next () ? ; Some (SectionIndex (index . get (self . file . endian) as usize)) } }
};
}
