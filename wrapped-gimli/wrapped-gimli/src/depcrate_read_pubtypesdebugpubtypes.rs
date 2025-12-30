// Generated macro for DebugPubTypes (struct)
macro_rules! Depcrate_read_pubtypesDebugPubTypes {
() => {
// Module: crate::read::pubtypes
// Provides: {"DebugPubTypes"}
// Dependencies: {}
# [doc = " The `DebugPubTypes` struct represents the DWARF public types information"] # [doc = " found in the `.debug_info` section."] # [derive (Debug , Clone)] pub struct DebugPubTypes < R : Reader > (DebugLookup < R , PubStuffParser < R , PubTypesEntry < R > > >) ;
};
}
