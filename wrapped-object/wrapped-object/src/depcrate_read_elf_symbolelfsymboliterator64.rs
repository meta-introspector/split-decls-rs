// Generated macro for ElfSymbolIterator64 (type)
macro_rules! Depcrate_read_elf_symbolElfSymbolIterator64 {
() => {
// Module: crate::read::elf::symbol
// Provides: {"ElfSymbolIterator64"}
// Dependencies: {}
# [doc = " An iterator for the symbols in an [`ElfFile64`](super::ElfFile64)."] pub type ElfSymbolIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSymbolIterator < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
};
}
