// Generated macro for PubNamesEntry (struct)
macro_rules! Depcrate_read_pubnamesPubNamesEntry {
() => {
// Module: crate::read::pubnames
// Provides: {"PubNamesEntry"}
// Dependencies: {}
# [doc = " A single parsed pubname."] # [derive (Debug , Clone)] pub struct PubNamesEntry < R : Reader > { unit_header_offset : DebugInfoOffset < R :: Offset > , die_offset : UnitOffset < R :: Offset > , name : R , }
};
}
