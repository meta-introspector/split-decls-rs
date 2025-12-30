// Generated macro for Range (struct)
macro_rules! Depcrate_read_rnglistsRange {
() => {
// Module: crate::read::rnglists
// Provides: {"Range"}
// Dependencies: {}
# [doc = " An address range from the `.debug_ranges`, `.debug_rnglists`, or `.debug_aranges` sections."] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Range { # [doc = " The beginning address of the range."] pub begin : u64 , # [doc = " The first address past the end of the range."] pub end : u64 , }
};
}
