// Generated macro for impl_441 (impl)
macro_rules! Depcrate_read_elf_sectionimpl_441 {
() => {
// Module: crate::read::elf::section
// Provides: {"impl_441"}
// Dependencies: {}
impl < 'data , 'file , Elf , R > ElfSectionIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { pub (super) fn new (file : & 'file ElfFile < 'data , Elf , R >) -> Self { let mut iter = file . sections . iter () . enumerate () ; iter . next () ; ElfSectionIterator { file , iter } } }
};
}
