// Generated macro for impl_466 (impl)
macro_rules! Depcrate_read_elf_symbolimpl_466 {
() => {
// Module: crate::read::elf::symbol
// Provides: {"impl_466"}
// Dependencies: {}
impl < 'data , Elf : FileHeader , R : ReadRef < 'data > > Default for SymbolTable < 'data , Elf , R > { fn default () -> Self { SymbolTable { section : SectionIndex (0) , string_section : SectionIndex (0) , shndx_section : SectionIndex (0) , symbols : & [] , strings : Default :: default () , shndx : & [] , } } }
};
}
