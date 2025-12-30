// Generated macro for ElfSegment64 (type)
macro_rules! Depcrate_read_elf_segmentElfSegment64 {
() => {
// Module: crate::read::elf::segment
// Provides: {"ElfSegment64"}
// Dependencies: {}
# [doc = " A segment in an [`ElfFile64`](super::ElfFile64)."] pub type ElfSegment64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSegment < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
};
}
