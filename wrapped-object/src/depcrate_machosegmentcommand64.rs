// Generated macro for SegmentCommand64 (struct)
macro_rules! Depcrate_machoSegmentCommand64 {
() => {
// Module: crate::macho
// Provides: {"SegmentCommand64"}
// Dependencies: {}
# [doc = " 64-bit segment load command."] # [doc = ""] # [doc = " The 64-bit segment load command indicates that a part of this file is to be"] # [doc = " mapped into a 64-bit task's address space.  If the 64-bit segment has"] # [doc = " sections then `Section64` structures directly follow the 64-bit segment"] # [doc = " command and their size is reflected in `cmdsize`."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct SegmentCommand64 < E : Endian > { # [doc = " LC_SEGMENT_64"] pub cmd : U32 < E > , # [doc = " includes sizeof section_64 structs"] pub cmdsize : U32 < E > , # [doc = " segment name"] pub segname : [u8 ; 16] , # [doc = " memory address of this segment"] pub vmaddr : U64 < E > , # [doc = " memory size of this segment"] pub vmsize : U64 < E > , # [doc = " file offset of this segment"] pub fileoff : U64 < E > , # [doc = " amount to map from the file"] pub filesize : U64 < E > , # [doc = " maximum VM protection"] pub maxprot : U32 < E > , # [doc = " initial VM protection"] pub initprot : U32 < E > , # [doc = " number of sections in segment"] pub nsects : U32 < E > , # [doc = " flags"] pub flags : U32 < E > , }
};
}
