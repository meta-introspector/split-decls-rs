// Generated macro for EdgeReferences (struct)
macro_rules! Depcrate_csrEdgeReferences {
() => {
// Module: crate::csr
// Provides: {"EdgeReferences"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct EdgeReferences < 'a , E : 'a , Ty , Ix : 'a > { source_index : NodeIndex < Ix > , index : usize , edge_ranges : Enumerate < Windows < 'a , usize > > , column : & 'a [NodeIndex < Ix >] , edges : & 'a [E] , iter : Zip < SliceIter < 'a , NodeIndex < Ix > > , SliceIter < 'a , E > > , ty : PhantomData < Ty > , }
};
}
