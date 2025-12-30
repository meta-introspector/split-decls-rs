// Generated macro for ElfSectionIterator32 (type)
macro_rules! Depcrate_read_elf_sectionElfSectionIterator32 {
() => {
// Module: crate::read::elf::section
// Provides: {"ElfSectionIterator32"}
// Dependencies: {}
# [doc = " An iterator for the sections in an [`ElfFile32`](super::ElfFile32)."] pub type ElfSectionIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSectionIterator < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
};
}
