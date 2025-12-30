// Generated macro for impl_177 (impl)
macro_rules! Depcrate_read_addrimpl_177 {
() => {
// Module: crate::read::addr
// Provides: {"impl_177"}
// Dependencies: {}
impl < R : Reader > AddrHeaderIter < R > { # [doc = " Advance the iterator to the next header."] pub fn next (& mut self) -> Result < Option < AddrHeader < R > > > { if self . input . is_empty () { return Ok (None) ; } let len = self . input . len () ; match AddrHeader :: parse (& mut self . input , self . offset) { Ok (header) => { self . offset . 0 += len - self . input . len () ; Ok (Some (header)) } Err (e) => { self . input . empty () ; Err (e) } } } }
};
}
