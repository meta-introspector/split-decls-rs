// Generated macro for impl_170 (impl)
macro_rules! Depcrate_dfa_sparseimpl_170 {
() => {
// Module: crate::dfa::sparse
// Provides: {"impl_170"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Seen { fn new () -> Seen { Seen { set : alloc :: collections :: BTreeSet :: new () } } fn insert (& mut self , id : StateID) { self . set . insert (id) ; } fn contains (& self , id : & StateID) -> bool { self . set . contains (id) } }
};
}
