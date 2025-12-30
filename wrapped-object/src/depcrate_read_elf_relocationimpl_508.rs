// Generated macro for impl_508 (impl)
macro_rules! Depcrate_read_elf_relocationimpl_508 {
() => {
// Module: crate::read::elf::relocation
// Provides: {"impl_508"}
// Dependencies: {}
impl < 'data , 'file , Elf , R > fmt :: Debug for ElfDynamicRelocationIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ElfDynamicRelocationIterator") . finish () } }
};
}
