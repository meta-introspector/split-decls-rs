// Generated macro for ElfComdatSectionIterator32 (type)
macro_rules! Depcrate_read_elf_comdatElfComdatSectionIterator32 {
() => {
// Module: crate::read::elf::comdat
// Provides: {"ElfComdatSectionIterator32"}
// Dependencies: {}
# [doc = " An iterator for the sections in a COMDAT section group in an [`ElfFile32`](super::ElfFile32)."] pub type ElfComdatSectionIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfComdatSectionIterator < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
};
}
