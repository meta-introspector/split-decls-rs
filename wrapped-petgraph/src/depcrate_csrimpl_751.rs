// Generated macro for impl_751 (impl)
macro_rules! Depcrate_csrimpl_751 {
() => {
// Module: crate::csr
// Provides: {"impl_751"}
// Dependencies: {}
impl < 'a , E , Ty , Ix > Iterator for Edges < 'a , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Item = EdgeReference < 'a , E , Ty , Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (move | (& j , w) | { let index = self . index ; self . index += 1 ; EdgeReference { index , source : self . source , target : j , weight : w , ty : PhantomData , } }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
