// Generated macro for AllEdgesMut (struct)
macro_rules! Depcrate_graphmapAllEdgesMut {
() => {
// Module: crate::graphmap
// Provides: {"AllEdgesMut"}
// Dependencies: {}
pub struct AllEdgesMut < 'a , N , E : 'a , Ty > where N : 'a + NodeTrait , { inner : IndexMapIterMut < 'a , (N , N) , E > , ty : PhantomData < Ty > , }
};
}
