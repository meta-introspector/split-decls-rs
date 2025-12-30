// Generated macro for ParAllEdgesMut (struct)
macro_rules! Depcrate_graphmapParAllEdgesMut {
() => {
// Module: crate::graphmap
// Provides: {"ParAllEdgesMut"}
// Dependencies: {}
# [doc = " A [ParallelIterator] over this graph's edges by mutable reference."] # [cfg (feature = "rayon")] pub struct ParAllEdgesMut < 'a , N , E : 'a , Ty > where N : NodeTrait + Send + Sync , E : Send , { inner : ParIterMut < 'a , (N , N) , E > , ty : PhantomData < fn (Ty) > , }
};
}
