// Generated macro for impl_167 (impl)
macro_rules! Depcrate_dfa_sparseimpl_167 {
() => {
// Module: crate::dfa::sparse
// Provides: {"impl_167"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Seen { fn new () -> Seen { Seen { set : alloc :: collections :: BTreeSet :: new () } } fn insert (& mut self , id : StateID) { self . set . insert (id) ; } fn contains (& self , id : & StateID) -> bool { self . set . contains (id) } }
};
}
