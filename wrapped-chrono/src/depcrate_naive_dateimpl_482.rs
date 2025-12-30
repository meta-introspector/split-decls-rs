// Generated macro for impl_482 (impl)
macro_rules! Depcrate_naive_dateimpl_482 {
() => {
// Module: crate::naive::date
// Provides: {"impl_482"}
// Dependencies: {}
impl DoubleEndedIterator for NaiveDateDaysIterator { fn next_back (& mut self) -> Option < Self :: Item > { let current = self . value ; self . value = current . pred_opt () ? ; Some (current) } }
};
}
