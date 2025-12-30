// Generated macro for impl_954 (impl)
macro_rules! Depcrate_weekday_setimpl_954 {
() => {
// Module: crate::weekday_set
// Provides: {"impl_954"}
// Dependencies: {}
impl Iterator for WeekdaySetIter { type Item = Weekday ; fn next (& mut self) -> Option < Self :: Item > { if self . days . is_empty () { return None ; } let (before , after) = self . days . split_at (self . start) ; let days = if after . is_empty () { before } else { after } ; let next = days . first () . expect ("the collection is not empty") ; self . days . remove (next) ; Some (next) } }
};
}
