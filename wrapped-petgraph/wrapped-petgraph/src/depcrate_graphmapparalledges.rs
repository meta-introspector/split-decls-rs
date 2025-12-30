// Generated macro for ParAllEdges (struct)
macro_rules! Depcrate_graphmapParAllEdges {
() => {
// Module: crate::graphmap
// Provides: {"ParAllEdges"}
// Dependencies: {}
# [doc = " A [ParallelIterator] over this graph's edges."] # [cfg (feature = "rayon")] pub struct ParAllEdges < 'a , N , E , Ty > where N : NodeTrait + Send + Sync , E : Sync , { inner : ParIter < 'a , (N , N) , E > , ty : PhantomData < fn (Ty) > , }
};
}
