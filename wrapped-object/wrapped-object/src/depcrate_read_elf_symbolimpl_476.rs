// Generated macro for impl_476 (impl)
macro_rules! Depcrate_read_elf_symbolimpl_476 {
() => {
// Module: crate::read::elf::symbol
// Provides: {"impl_476"}
// Dependencies: {}
impl < 'data , 'file , Elf , R > ElfSymbolIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { pub (super) fn new (endian : Elf :: Endian , symbols : & 'file SymbolTable < 'data , Elf , R >) -> Self { ElfSymbolIterator { endian , symbols , index : SymbolIndex (1) , } } }
};
}
