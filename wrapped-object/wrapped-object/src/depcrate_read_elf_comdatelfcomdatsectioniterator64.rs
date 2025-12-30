// Generated macro for ElfComdatSectionIterator64 (type)
macro_rules! Depcrate_read_elf_comdatElfComdatSectionIterator64 {
() => {
// Module: crate::read::elf::comdat
// Provides: {"ElfComdatSectionIterator64"}
// Dependencies: {}
# [doc = " An iterator for the sections in a COMDAT section group in an [`ElfFile64`](super::ElfFile64)."] pub type ElfComdatSectionIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfComdatSectionIterator < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
};
}
