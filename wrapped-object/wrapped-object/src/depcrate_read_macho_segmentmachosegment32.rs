// Generated macro for MachOSegment32 (type)
macro_rules! Depcrate_read_macho_segmentMachOSegment32 {
() => {
// Module: crate::read::macho::segment
// Provides: {"MachOSegment32"}
// Dependencies: {}
# [doc = " A segment in a [`MachOFile32`](super::MachOFile32)."] pub type MachOSegment32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSegment < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
};
}
