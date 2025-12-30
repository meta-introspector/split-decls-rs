// Generated macro for EdgeReference (struct)
macro_rules! Depcrate_csrEdgeReference {
() => {
// Module: crate::csr
// Provides: {"EdgeReference"}
// Dependencies: {}
# [derive (Debug)] pub struct EdgeReference < 'a , E : 'a , Ty , Ix : 'a = DefaultIx > { index : EdgeIndex , source : NodeIndex < Ix > , target : NodeIndex < Ix > , weight : & 'a E , ty : PhantomData < Ty > , }
};
}
