// Generated macro for ElfComdatIterator32 (type)
macro_rules! Depcrate_read_elf_comdatElfComdatIterator32 {
() => {
// Module: crate::read::elf::comdat
// Provides: {"ElfComdatIterator32"}
// Dependencies: {}
# [doc = " An iterator for the COMDAT section groups in an [`ElfFile32`](super::ElfFile32)."] pub type ElfComdatIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfComdatIterator < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
};
}
