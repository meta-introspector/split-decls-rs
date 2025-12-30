// Generated macro for impl_667 (impl)
macro_rules! Depcrate_read_pubnamesimpl_667 {
() => {
// Module: crate::read::pubnames
// Provides: {"impl_667"}
// Dependencies: {}
impl < R : Reader > PubStuffEntry < R > for PubNamesEntry < R > { fn new (die_offset : UnitOffset < R :: Offset > , name : R , unit_header_offset : DebugInfoOffset < R :: Offset > ,) -> Self { PubNamesEntry { unit_header_offset , die_offset , name , } } }
};
}
