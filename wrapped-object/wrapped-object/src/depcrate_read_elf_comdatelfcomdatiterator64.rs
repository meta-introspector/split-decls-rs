// Generated macro for ElfComdatIterator64 (type)
macro_rules! Depcrate_read_elf_comdatElfComdatIterator64 {
() => {
// Module: crate::read::elf::comdat
// Provides: {"ElfComdatIterator64"}
// Dependencies: {}
# [doc = " An iterator for the COMDAT section groups in an [`ElfFile64`](super::ElfFile64)."] pub type ElfComdatIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfComdatIterator < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
};
}
