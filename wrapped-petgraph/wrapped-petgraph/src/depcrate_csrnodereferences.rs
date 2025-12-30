// Generated macro for NodeReferences (struct)
macro_rules! Depcrate_csrNodeReferences {
() => {
// Module: crate::csr
// Provides: {"NodeReferences"}
// Dependencies: {}
# [doc = " Iterator over all nodes of a graph."] # [derive (Debug , Clone)] pub struct NodeReferences < 'a , N : 'a , Ix : IndexType = DefaultIx > { iter : Enumerate < SliceIter < 'a , N > > , ty : PhantomData < Ix > , }
};
}
