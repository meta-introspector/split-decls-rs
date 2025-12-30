// Generated macro for impl_176 (impl)
macro_rules! Depcrate_store_impls_loose_iterimpl_176 {
() => {
// Module: crate::store_impls::loose::iter
// Provides: {"impl_176"}
// Dependencies: {}
impl Iterator for loose :: Iter { type Item = Result < gix_hash :: ObjectId , Error > ; fn next (& mut self) -> Option < Self :: Item > { while let Some (res) = self . inner . next () { if let Some (res) = self . path_to_id (res) { return Some (res) ; } } None } }
};
}
