// Generated macro for Neighbors (struct)
macro_rules! Depcrate_graphmapNeighbors {
() => {
// Module: crate::graphmap
// Provides: {"Neighbors"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct Neighbors < 'a , N , Ty = Undirected > where N : 'a , Ty : EdgeType , { iter : Iter < 'a , (N , CompactDirection) > , ty : PhantomData < Ty > , }
};
}
