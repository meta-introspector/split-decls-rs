// Generated macro for RangeListTable (struct)
macro_rules! Depcrate_write_rangeRangeListTable {
() => {
// Module: crate::write::range
// Provides: {"RangeListTable"}
// Dependencies: {}
# [doc = " A table of range lists that will be stored in a `.debug_ranges` or `.debug_rnglists` section."] # [derive (Debug , Default)] pub struct RangeListTable { base_id : BaseId , ranges : FnvIndexSet < RangeList > , }
};
}
