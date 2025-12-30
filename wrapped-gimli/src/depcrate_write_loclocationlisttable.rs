// Generated macro for LocationListTable (struct)
macro_rules! Depcrate_write_locLocationListTable {
() => {
// Module: crate::write::loc
// Provides: {"LocationListTable"}
// Dependencies: {}
# [doc = " A table of location lists that will be stored in a `.debug_loc` or `.debug_loclists` section."] # [derive (Debug , Default)] pub struct LocationListTable { base_id : BaseId , locations : FnvIndexSet < LocationList > , }
};
}
