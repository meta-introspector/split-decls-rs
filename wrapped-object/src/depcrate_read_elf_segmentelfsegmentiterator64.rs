// Generated macro for ElfSegmentIterator64 (type)
macro_rules! Depcrate_read_elf_segmentElfSegmentIterator64 {
() => {
// Module: crate::read::elf::segment
// Provides: {"ElfSegmentIterator64"}
// Dependencies: {}
# [doc = " An iterator for the segments in an [`ElfFile64`](super::ElfFile64)."] pub type ElfSegmentIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSegmentIterator < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
};
}
