// Generated macro for impl_125 (impl)
macro_rules! Depcrate_visitimpl_125 {
() => {
// Module: crate::visit
// Provides: {"impl_125"}
// Dependencies: {}
# [cfg (feature = "std")] impl < N , S > VisitMap < N > for std :: collections :: HashSet < N , S > where N : Hash + Eq , S : BuildHasher , { fn visit (& mut self , x : N) -> bool { self . insert (x) } fn is_visited (& self , x : & N) -> bool { self . contains (x) } fn unvisit (& mut self , x : N) -> bool { self . remove (& x) } }
};
}
