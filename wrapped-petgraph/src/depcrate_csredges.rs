// Generated macro for Edges (struct)
macro_rules! Depcrate_csrEdges {
() => {
// Module: crate::csr
// Provides: {"Edges"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct Edges < 'a , E : 'a , Ty = Directed , Ix : 'a = DefaultIx > { index : usize , source : NodeIndex < Ix > , iter : Zip < SliceIter < 'a , NodeIndex < Ix > > , SliceIter < 'a , E > > , ty : PhantomData < Ty > , }
};
}
