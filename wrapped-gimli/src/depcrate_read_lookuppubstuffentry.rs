// Generated macro for PubStuffEntry (trait)
macro_rules! Depcrate_read_lookupPubStuffEntry {
() => {
// Module: crate::read::lookup
// Provides: {"PubStuffEntry"}
// Dependencies: {}
pub trait PubStuffEntry < R : Reader > { fn new (die_offset : UnitOffset < R :: Offset > , name : R , unit_header_offset : DebugInfoOffset < R :: Offset > ,) -> Self ; }
};
}
