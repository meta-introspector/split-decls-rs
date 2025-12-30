// Generated macro for DebugPubNames (struct)
macro_rules! Depcrate_read_pubnamesDebugPubNames {
() => {
// Module: crate::read::pubnames
// Provides: {"DebugPubNames"}
// Dependencies: {}
# [doc = " The `DebugPubNames` struct represents the DWARF public names information"] # [doc = " found in the `.debug_pubnames` section."] # [derive (Debug , Clone)] pub struct DebugPubNames < R : Reader > (DebugLookup < R , PubStuffParser < R , PubNamesEntry < R > > >) ;
};
}
