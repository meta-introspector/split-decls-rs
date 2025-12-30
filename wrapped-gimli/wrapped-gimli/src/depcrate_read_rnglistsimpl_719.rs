// Generated macro for impl_719 (impl)
macro_rules! Depcrate_read_rnglistsimpl_719 {
() => {
// Module: crate::read::rnglists
// Provides: {"impl_719"}
// Dependencies: {}
impl < R : Reader > RawRngListIter < R > { # [doc = " Construct a `RawRngListIter`."] fn new (input : R , encoding : Encoding , format : RangeListsFormat) -> RawRngListIter < R > { RawRngListIter { input , encoding , format , } } # [doc = " Advance the iterator to the next range."] pub fn next (& mut self) -> Result < Option < RawRngListEntry < R :: Offset > > > { if self . input . is_empty () { return Ok (None) ; } match RawRngListEntry :: parse (& mut self . input , self . encoding , self . format) { Ok (range) => { if range . is_none () { self . input . empty () ; } Ok (range) } Err (e) => { self . input . empty () ; Err (e) } } } }
};
}
