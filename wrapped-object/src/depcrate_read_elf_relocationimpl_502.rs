// Generated macro for impl_502 (impl)
macro_rules! Depcrate_read_elf_relocationimpl_502 {
() => {
// Module: crate::read::elf::relocation
// Provides: {"impl_502"}
// Dependencies: {}
impl < 'data , Elf : FileHeader > ElfRelocationIterator < 'data , Elf > { fn is_rel (& self) -> bool { match self { ElfRelocationIterator :: Rel (..) => true , ElfRelocationIterator :: Rela (..) => false , ElfRelocationIterator :: Crel (i) => ! i . is_rela () , } } }
};
}
