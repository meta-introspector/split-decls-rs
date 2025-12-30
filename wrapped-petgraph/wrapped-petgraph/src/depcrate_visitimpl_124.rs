// Generated macro for impl_124 (impl)
macro_rules! Depcrate_visitimpl_124 {
() => {
// Module: crate::visit
// Provides: {"impl_124"}
// Dependencies: {}
impl < N , S > VisitMap < N > for HashSet < N , S > where N : Hash + Eq , S : BuildHasher , { fn visit (& mut self , x : N) -> bool { self . insert (x) } fn is_visited (& self , x : & N) -> bool { self . contains (x) } fn unvisit (& mut self , x : N) -> bool { self . remove (& x) } }
};
}
