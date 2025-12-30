// Generated macro for impl_47 (impl)
macro_rules! Depcrate_commit_topo_iterimpl_47 {
() => {
// Module: crate::commit::topo::iter
// Provides: {"impl_47"}
// Dependencies: {}
impl < Find , Predicate > Iterator for Topo < Find , Predicate > where Find : gix_object :: Find , Predicate : FnMut (& oid) -> bool , { type Item = Result < Info , Error > ; fn next (& mut self) -> Option < Self :: Item > { loop { match self . pop_commit () ? { Ok (id) => { if (self . predicate) (& id . id) { return Some (Ok (id)) ; } } Err (e) => return Some (Err (e)) , } } } }
};
}
