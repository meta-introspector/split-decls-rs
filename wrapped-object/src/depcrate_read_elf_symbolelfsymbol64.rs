// Generated macro for ElfSymbol64 (type)
macro_rules! Depcrate_read_elf_symbolElfSymbol64 {
() => {
// Module: crate::read::elf::symbol
// Provides: {"ElfSymbol64"}
// Dependencies: {}
# [doc = " A symbol in an [`ElfFile64`](super::ElfFile64)."] pub type ElfSymbol64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSymbol < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
};
}
