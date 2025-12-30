// Generated macro for SegmentFlags (enum)
macro_rules! Depcrate_commonSegmentFlags {
() => {
// Module: crate::common
// Provides: {"SegmentFlags"}
// Dependencies: {}
# [doc = " Segment flags that are specific to each file format."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum SegmentFlags { # [doc = " No segment flags."] None , # [doc = " ELF segment flags."] Elf { # [doc = " `p_flags` field in the segment header."] p_flags : u32 , } , # [doc = " Mach-O segment flags."] MachO { # [doc = " `flags` field in the segment header."] flags : u32 , # [doc = " `maxprot` field in the segment header."] maxprot : u32 , # [doc = " `initprot` field in the segment header."] initprot : u32 , } , # [doc = " COFF segment flags."] Coff { # [doc = " `Characteristics` field in the segment header."] characteristics : u32 , } , }
};
}
