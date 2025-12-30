// Generated macro for impl_684 (impl)
macro_rules! Depcrate_read_pubtypesimpl_684 {
() => {
// Module: crate::read::pubtypes
// Provides: {"impl_684"}
// Dependencies: {}
impl < R : Reader > PubStuffEntry < R > for PubTypesEntry < R > { fn new (die_offset : UnitOffset < R :: Offset > , name : R , unit_header_offset : DebugInfoOffset < R :: Offset > ,) -> Self { PubTypesEntry { unit_header_offset , die_offset , name , } } }
};
}
