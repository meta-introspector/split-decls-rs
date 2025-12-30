// Generated macro for ElfSymbol32 (type)
macro_rules! Depcrate_read_elf_symbolElfSymbol32 {
() => {
// Module: crate::read::elf::symbol
// Provides: {"ElfSymbol32"}
// Dependencies: {}
# [doc = " A symbol in an [`ElfFile32`](super::ElfFile32)."] pub type ElfSymbol32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSymbol < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
};
}
