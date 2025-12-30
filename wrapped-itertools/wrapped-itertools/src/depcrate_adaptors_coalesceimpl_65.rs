// Generated macro for impl_65 (impl)
macro_rules! Depcrate_adaptors_coalesceimpl_65 {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"impl_65"}
// Dependencies: {}
impl < T , F : FnMut (& T , & T) -> bool > DedupPredicate < T > for F { fn dedup_pair (& mut self , a : & T , b : & T) -> bool { self (a , b) } }
};
}
