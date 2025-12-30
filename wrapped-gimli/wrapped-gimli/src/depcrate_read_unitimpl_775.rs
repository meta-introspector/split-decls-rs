// Generated macro for impl_775 (impl)
macro_rules! Depcrate_read_unitimpl_775 {
() => {
// Module: crate::read::unit
// Provides: {"impl_775"}
// Dependencies: {}
impl < R : Reader > DebugInfoUnitHeadersIter < R > { # [doc = " Advance the iterator to the next unit header."] pub fn next (& mut self) -> Result < Option < UnitHeader < R > > > { if self . input . is_empty () { Ok (None) } else { let len = self . input . len () ; match parse_unit_header (& mut self . input , SectionId :: DebugInfo , self . offset) { Ok (header) => { self . offset . 0 += len - self . input . len () ; Ok (Some (header)) } Err (e) => { self . input . empty () ; Err (e) } } } } }
};
}
