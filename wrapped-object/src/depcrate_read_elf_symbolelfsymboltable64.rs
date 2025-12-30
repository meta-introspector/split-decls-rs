// Generated macro for ElfSymbolTable64 (type)
macro_rules! Depcrate_read_elf_symbolElfSymbolTable64 {
() => {
// Module: crate::read::elf::symbol
// Provides: {"ElfSymbolTable64"}
// Dependencies: {}
# [doc = " A symbol table in an [`ElfFile32`](super::ElfFile32)."] pub type ElfSymbolTable64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSymbolTable < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
};
}
