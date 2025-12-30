// Generated macro for MachOSegment64 (type)
macro_rules! Depcrate_read_macho_segmentMachOSegment64 {
() => {
// Module: crate::read::macho::segment
// Provides: {"MachOSegment64"}
// Dependencies: {}
# [doc = " A segment in a [`MachOFile64`](super::MachOFile64)."] pub type MachOSegment64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSegment < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
};
}
