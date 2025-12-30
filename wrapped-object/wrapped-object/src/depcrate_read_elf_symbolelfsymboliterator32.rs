// Generated macro for ElfSymbolIterator32 (type)
macro_rules! Depcrate_read_elf_symbolElfSymbolIterator32 {
() => {
// Module: crate::read::elf::symbol
// Provides: {"ElfSymbolIterator32"}
// Dependencies: {}
# [doc = " An iterator for the symbols in an [`ElfFile32`](super::ElfFile32)."] pub type ElfSymbolIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSymbolIterator < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
};
}
