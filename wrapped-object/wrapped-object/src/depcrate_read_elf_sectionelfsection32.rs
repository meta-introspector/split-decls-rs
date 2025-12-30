// Generated macro for ElfSection32 (type)
macro_rules! Depcrate_read_elf_sectionElfSection32 {
() => {
// Module: crate::read::elf::section
// Provides: {"ElfSection32"}
// Dependencies: {}
# [doc = " A section in an [`ElfFile32`](super::ElfFile32)."] pub type ElfSection32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSection < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
};
}
