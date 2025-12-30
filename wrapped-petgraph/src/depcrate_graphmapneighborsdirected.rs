// Generated macro for NeighborsDirected (struct)
macro_rules! Depcrate_graphmapNeighborsDirected {
() => {
// Module: crate::graphmap
// Provides: {"NeighborsDirected"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct NeighborsDirected < 'a , N , Ty > where N : 'a , Ty : EdgeType , { iter : Iter < 'a , (N , CompactDirection) > , start_node : N , dir : Direction , ty : PhantomData < Ty > , }
};
}
