// Generated macro for impl_487 (impl)
macro_rules! Depcrate_naive_dateimpl_487 {
() => {
// Module: crate::naive::date
// Provides: {"impl_487"}
// Dependencies: {}
impl DoubleEndedIterator for NaiveDateWeeksIterator { fn next_back (& mut self) -> Option < Self :: Item > { let current = self . value ; self . value = current . checked_sub_days (Days :: new (7)) ? ; Some (current) } }
};
}
