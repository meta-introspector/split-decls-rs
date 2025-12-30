// Generated macro for impl_485 (impl)
macro_rules! Depcrate_naive_dateimpl_485 {
() => {
// Module: crate::naive::date
// Provides: {"impl_485"}
// Dependencies: {}
impl Iterator for NaiveDateWeeksIterator { type Item = NaiveDate ; fn next (& mut self) -> Option < Self :: Item > { let current = self . value ; self . value = current . checked_add_days (Days :: new (7)) ? ; Some (current) } fn size_hint (& self) -> (usize , Option < usize >) { let exact_size = NaiveDate :: MAX . signed_duration_since (self . value) . num_weeks () ; (exact_size as usize , Some (exact_size as usize)) } }
};
}
