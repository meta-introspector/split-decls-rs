// Generated macro for impl_955 (impl)
macro_rules! Depcrate_weekday_setimpl_955 {
() => {
// Module: crate::weekday_set
// Provides: {"impl_955"}
// Dependencies: {}
impl DoubleEndedIterator for WeekdaySetIter { fn next_back (& mut self) -> Option < Self :: Item > { if self . days . is_empty () { return None ; } let (before , after) = self . days . split_at (self . start) ; let days = if before . is_empty () { after } else { before } ; let next_back = days . last () . expect ("the collection is not empty") ; self . days . remove (next_back) ; Some (next_back) } }
};
}
