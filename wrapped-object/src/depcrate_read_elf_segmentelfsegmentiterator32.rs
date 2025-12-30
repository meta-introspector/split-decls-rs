// Generated macro for ElfSegmentIterator32 (type)
macro_rules! Depcrate_read_elf_segmentElfSegmentIterator32 {
() => {
// Module: crate::read::elf::segment
// Provides: {"ElfSegmentIterator32"}
// Dependencies: {}
# [doc = " An iterator for the segments in an [`ElfFile32`](super::ElfFile32)."] pub type ElfSegmentIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSegmentIterator < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
};
}
