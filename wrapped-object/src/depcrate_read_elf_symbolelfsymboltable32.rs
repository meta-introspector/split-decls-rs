// Generated macro for ElfSymbolTable32 (type)
macro_rules! Depcrate_read_elf_symbolElfSymbolTable32 {
() => {
// Module: crate::read::elf::symbol
// Provides: {"ElfSymbolTable32"}
// Dependencies: {}
# [doc = " A symbol table in an [`ElfFile32`](super::ElfFile32)."] pub type ElfSymbolTable32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSymbolTable < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
};
}
