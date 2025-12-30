// Generated macro for ElfSection64 (type)
macro_rules! Depcrate_read_elf_sectionElfSection64 {
() => {
// Module: crate::read::elf::section
// Provides: {"ElfSection64"}
// Dependencies: {}
# [doc = " A section in an [`ElfFile64`](super::ElfFile64)."] pub type ElfSection64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSection < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
};
}
