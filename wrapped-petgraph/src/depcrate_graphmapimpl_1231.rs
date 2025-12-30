// Generated macro for impl_1231 (impl)
macro_rules! Depcrate_graphmapimpl_1231 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1231"}
// Dependencies: {}
impl < N , E , Ty , S > visit :: Visitable for GraphMap < N , E , Ty , S > where N : Copy + Ord + Hash , Ty : EdgeType , S : BuildHasher , { type Map = HashSet < N > ; fn visit_map (& self) -> HashSet < N > { HashSet :: with_capacity (self . node_count ()) } fn reset_map (& self , map : & mut Self :: Map) { map . clear () ; } }
};
}
