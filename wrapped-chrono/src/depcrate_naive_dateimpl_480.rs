// Generated macro for impl_480 (impl)
macro_rules! Depcrate_naive_dateimpl_480 {
() => {
// Module: crate::naive::date
// Provides: {"impl_480"}
// Dependencies: {}
impl Iterator for NaiveDateDaysIterator { type Item = NaiveDate ; fn next (& mut self) -> Option < Self :: Item > { let current = self . value ; self . value = current . succ_opt () ? ; Some (current) } fn size_hint (& self) -> (usize , Option < usize >) { let exact_size = NaiveDate :: MAX . signed_duration_since (self . value) . num_days () ; (exact_size as usize , Some (exact_size as usize)) } }
};
}
