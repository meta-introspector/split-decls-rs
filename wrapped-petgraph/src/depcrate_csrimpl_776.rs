// Generated macro for impl_776 (impl)
macro_rules! Depcrate_csrimpl_776 {
() => {
// Module: crate::csr
// Provides: {"impl_776"}
// Dependencies: {}
impl < 'a , N , Ix > Iterator for NodeReferences < 'a , N , Ix > where Ix : IndexType , { type Item = (NodeIndex < Ix > , & 'a N) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (i , weight) | (Ix :: new (i) , weight)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
