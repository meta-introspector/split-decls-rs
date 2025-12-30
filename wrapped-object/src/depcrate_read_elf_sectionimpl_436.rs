// Generated macro for impl_436 (impl)
macro_rules! Depcrate_read_elf_sectionimpl_436 {
() => {
// Module: crate::read::elf::section
// Provides: {"impl_436"}
// Dependencies: {}
impl < 'data , Elf : FileHeader , R : ReadRef < 'data > > Default for SectionTable < 'data , Elf , R > { fn default () -> Self { SectionTable { sections : & [] , strings : StringTable :: default () , } } }
};
}
