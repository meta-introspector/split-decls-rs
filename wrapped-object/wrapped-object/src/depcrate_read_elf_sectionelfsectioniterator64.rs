// Generated macro for ElfSectionIterator64 (type)
macro_rules! Depcrate_read_elf_sectionElfSectionIterator64 {
() => {
// Module: crate::read::elf::section
// Provides: {"ElfSectionIterator64"}
// Dependencies: {}
# [doc = " An iterator for the sections in an [`ElfFile64`](super::ElfFile64)."] pub type ElfSectionIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSectionIterator < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
};
}
