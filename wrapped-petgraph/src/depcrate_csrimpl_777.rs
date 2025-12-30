// Generated macro for impl_777 (impl)
macro_rules! Depcrate_csrimpl_777 {
() => {
// Module: crate::csr
// Provides: {"impl_777"}
// Dependencies: {}
impl < N , Ix > DoubleEndedIterator for NodeReferences < '_ , N , Ix > where Ix : IndexType , { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (| (i , weight) | (Ix :: new (i) , weight)) } }
};
}
