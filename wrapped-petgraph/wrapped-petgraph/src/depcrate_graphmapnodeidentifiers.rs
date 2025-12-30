// Generated macro for NodeIdentifiers (struct)
macro_rules! Depcrate_graphmapNodeIdentifiers {
() => {
// Module: crate::graphmap
// Provides: {"NodeIdentifiers"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct NodeIdentifiers < 'a , N , E : 'a , Ty > where N : 'a + NodeTrait , { iter : IndexMapIter < 'a , N , Vec < (N , CompactDirection) > > , ty : PhantomData < Ty > , edge_ty : PhantomData < E > , }
};
}
