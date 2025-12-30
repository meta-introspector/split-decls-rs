// Generated macro for ElfSymbolIterator (struct)
macro_rules! Depcrate_read_elf_symbolElfSymbolIterator {
() => {
// Module: crate::read::elf::symbol
// Provides: {"ElfSymbolIterator"}
// Dependencies: {}
# [doc = " An iterator for the symbols in an [`ElfFile`](super::ElfFile)."] pub struct ElfSymbolIterator < 'data , 'file , Elf , R = & 'data [u8] > where Elf : FileHeader , R : ReadRef < 'data > , { endian : Elf :: Endian , symbols : & 'file SymbolTable < 'data , Elf , R > , index : SymbolIndex , }
};
}
