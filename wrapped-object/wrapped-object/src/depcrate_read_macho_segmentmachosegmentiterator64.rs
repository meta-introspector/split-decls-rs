// Generated macro for MachOSegmentIterator64 (type)
macro_rules! Depcrate_read_macho_segmentMachOSegmentIterator64 {
() => {
// Module: crate::read::macho::segment
// Provides: {"MachOSegmentIterator64"}
// Dependencies: {}
# [doc = " An iterator for the segments in a [`MachOFile64`](super::MachOFile64)."] pub type MachOSegmentIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSegmentIterator < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
};
}
