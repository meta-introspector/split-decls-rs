// Generated macro for impl_326 (impl)
macro_rules! Depcrate_read_dwarfimpl_326 {
() => {
// Module: crate::read::dwarf
// Provides: {"impl_326"}
// Dependencies: {}
impl < R : Reader > RangeIter < R > { # [doc = " Advance the iterator to the next range."] pub fn next (& mut self) -> Result < Option < Range > > { match self . 0 { RangeIterInner :: Single (ref mut range) => Ok (range . take ()) , RangeIterInner :: List (ref mut list) => list . next () , } } }
};
}
