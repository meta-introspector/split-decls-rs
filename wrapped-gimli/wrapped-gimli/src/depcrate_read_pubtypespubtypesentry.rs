// Generated macro for PubTypesEntry (struct)
macro_rules! Depcrate_read_pubtypesPubTypesEntry {
() => {
// Module: crate::read::pubtypes
// Provides: {"PubTypesEntry"}
// Dependencies: {}
# [doc = " A single parsed pubtype."] # [derive (Debug , Clone)] pub struct PubTypesEntry < R : Reader > { unit_header_offset : DebugInfoOffset < R :: Offset > , die_offset : UnitOffset < R :: Offset > , name : R , }
};
}
