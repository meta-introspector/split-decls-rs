// Generated macro for RawRangeListsOffset (struct)
macro_rules! Depcrate_commonRawRangeListsOffset {
() => {
// Module: crate::common
// Provides: {"RawRangeListsOffset"}
// Dependencies: {}
# [doc = " An offset into either the `.debug_ranges` section or the `.debug_rnglists` section,"] # [doc = " depending on the version of the unit the offset was contained in."] # [doc = ""] # [doc = " If this is from a DWARF 4 DWO file, then it must additionally be offset by the"] # [doc = " value of `DW_AT_GNU_ranges_base`. You can use `Dwarf::ranges_offset_from_raw` to do this."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct RawRangeListsOffset < T = usize > (pub T) ;
};
}
