// Generated macro for NodeReferences (struct)
macro_rules! Depcrate_graphmapNodeReferences {
() => {
// Module: crate::graphmap
// Provides: {"NodeReferences"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct NodeReferences < 'a , N , E : 'a , Ty > where N : 'a + NodeTrait , { iter : IndexMapIter < 'a , N , Vec < (N , CompactDirection) > > , ty : PhantomData < Ty > , edge_ty : PhantomData < E > , }
};
}
