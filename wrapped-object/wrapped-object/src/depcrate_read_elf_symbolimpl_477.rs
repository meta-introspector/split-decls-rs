// Generated macro for impl_477 (impl)
macro_rules! Depcrate_read_elf_symbolimpl_477 {
() => {
// Module: crate::read::elf::symbol
// Provides: {"impl_477"}
// Dependencies: {}
impl < 'data , 'file , Elf : FileHeader , R : ReadRef < 'data > > fmt :: Debug for ElfSymbolIterator < 'data , 'file , Elf , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ElfSymbolIterator") . finish () } }
};
}
