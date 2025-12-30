// Generated macro for ElfSegment32 (type)
macro_rules! Depcrate_read_elf_segmentElfSegment32 {
() => {
// Module: crate::read::elf::segment
// Provides: {"ElfSegment32"}
// Dependencies: {}
# [doc = " A segment in an [`ElfFile32`](super::ElfFile32)."] pub type ElfSegment32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSegment < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
};
}
