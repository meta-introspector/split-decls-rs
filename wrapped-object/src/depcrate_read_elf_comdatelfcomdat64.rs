// Generated macro for ElfComdat64 (type)
macro_rules! Depcrate_read_elf_comdatElfComdat64 {
() => {
// Module: crate::read::elf::comdat
// Provides: {"ElfComdat64"}
// Dependencies: {}
# [doc = " A COMDAT section group in an [`ElfFile64`](super::ElfFile64)."] pub type ElfComdat64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfComdat < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
};
}
