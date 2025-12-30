// Generated macro for impl_513 (impl)
macro_rules! Depcrate_read_elf_relocationimpl_513 {
() => {
// Module: crate::read::elf::relocation
// Provides: {"impl_513"}
// Dependencies: {}
impl < 'data , 'file , Elf , R > fmt :: Debug for ElfSectionRelocationIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ElfSectionRelocationIterator") . finish () } }
};
}
