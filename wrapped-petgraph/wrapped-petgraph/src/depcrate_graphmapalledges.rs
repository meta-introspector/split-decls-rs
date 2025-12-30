// Generated macro for AllEdges (struct)
macro_rules! Depcrate_graphmapAllEdges {
() => {
// Module: crate::graphmap
// Provides: {"AllEdges"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct AllEdges < 'a , N , E : 'a , Ty > where N : 'a + NodeTrait , { inner : IndexMapIter < 'a , (N , N) , E > , ty : PhantomData < Ty > , }
};
}
