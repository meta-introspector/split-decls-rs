// Generated macro for impl_442 (impl)
macro_rules! Depcrate_read_elf_sectionimpl_442 {
() => {
// Module: crate::read::elf::section
// Provides: {"impl_442"}
// Dependencies: {}
impl < 'data , 'file , Elf , R > Iterator for ElfSectionIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { type Item = ElfSection < 'data , 'file , Elf , R > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (index , section) | ElfSection { index : SectionIndex (index) , file : self . file , section , }) } }
};
}
