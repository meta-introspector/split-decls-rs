// Generated macro for MachOSegment (struct)
macro_rules! Depcrate_read_macho_segmentMachOSegment {
() => {
// Module: crate::read::macho::segment
// Provides: {"MachOSegment"}
// Dependencies: {}
# [doc = " A segment in a [`MachOFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSegment`] trait implementation."] # [derive (Debug)] pub struct MachOSegment < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { file : & 'file MachOFile < 'data , Mach , R > , internal : & 'file MachOSegmentInternal < 'data , Mach , R > , }
};
}
