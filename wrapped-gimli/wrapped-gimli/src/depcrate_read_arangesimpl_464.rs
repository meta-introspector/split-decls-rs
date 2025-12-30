// Generated macro for impl_464 (impl)
macro_rules! Depcrate_read_arangesimpl_464 {
() => {
// Module: crate::read::aranges
// Provides: {"impl_464"}
// Dependencies: {}
impl < R : Reader > ArangeHeaderIter < R > { # [doc = " Advance the iterator to the next header."] pub fn next (& mut self) -> Result < Option < ArangeHeader < R > > > { if self . input . is_empty () { return Ok (None) ; } let len = self . input . len () ; match ArangeHeader :: parse (& mut self . input , self . offset) { Ok (header) => { self . offset . 0 += len - self . input . len () ; Ok (Some (header)) } Err (e) => { self . input . empty () ; Err (e) } } } }
};
}
