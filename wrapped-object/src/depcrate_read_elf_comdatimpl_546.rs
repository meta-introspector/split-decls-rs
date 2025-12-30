// Generated macro for impl_546 (impl)
macro_rules! Depcrate_read_elf_comdatimpl_546 {
() => {
// Module: crate::read::elf::comdat
// Provides: {"impl_546"}
// Dependencies: {}
impl < 'data , 'file , Elf , R > Iterator for ElfComdatIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { type Item = ElfComdat < 'data , 'file , Elf , R > ; fn next (& mut self) -> Option < Self :: Item > { for (_index , section) in self . iter . by_ref () { if let Some (comdat) = ElfComdat :: parse (self . file , section) { return Some (comdat) ; } } None } }
};
}
