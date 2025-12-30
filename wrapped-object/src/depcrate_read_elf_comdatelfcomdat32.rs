// Generated macro for ElfComdat32 (type)
macro_rules! Depcrate_read_elf_comdatElfComdat32 {
() => {
// Module: crate::read::elf::comdat
// Provides: {"ElfComdat32"}
// Dependencies: {}
# [doc = " A COMDAT section group in an [`ElfFile32`](super::ElfFile32)."] pub type ElfComdat32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfComdat < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
};
}
