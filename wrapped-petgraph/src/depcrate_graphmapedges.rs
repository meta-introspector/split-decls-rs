// Generated macro for Edges (struct)
macro_rules! Depcrate_graphmapEdges {
() => {
// Module: crate::graphmap
// Provides: {"Edges"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct Edges < 'a , N , E : 'a , Ty , # [cfg (not (feature = "std"))] S , # [cfg (feature = "std")] S = RandomState , > where N : 'a + NodeTrait , Ty : EdgeType , S : BuildHasher , { from : N , edges : & 'a IndexMap < (N , N) , E , S > , iter : Neighbors < 'a , N , Ty > , }
};
}
