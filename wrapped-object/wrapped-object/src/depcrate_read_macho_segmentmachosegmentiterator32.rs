// Generated macro for MachOSegmentIterator32 (type)
macro_rules! Depcrate_read_macho_segmentMachOSegmentIterator32 {
() => {
// Module: crate::read::macho::segment
// Provides: {"MachOSegmentIterator32"}
// Dependencies: {}
# [doc = " An iterator for the segments in a [`MachOFile32`](super::MachOFile32)."] pub type MachOSegmentIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSegmentIterator < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
};
}
