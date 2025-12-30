// Generated macro for impl_447 (impl)
macro_rules! Depcrate_kmerge_implimpl_447 {
() => {
// Module: crate::kmerge_impl
// Provides: {"impl_447"}
// Dependencies: {}
impl < T , F : FnMut (& T , & T) -> bool > KMergePredicate < T > for F { fn kmerge_pred (& mut self , a : & T , b : & T) -> bool { self (a , b) } }
};
}
