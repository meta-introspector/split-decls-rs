// Generated macro for impl_759 (impl)
macro_rules! Depcrate_zip_eq_implimpl_759 {
() => {
// Module: crate::zip_eq_impl
// Provides: {"impl_759"}
// Dependencies: {}
impl < I , J > Iterator for ZipEq < I , J > where I : Iterator , J : Iterator , { type Item = (I :: Item , J :: Item) ; fn next (& mut self) -> Option < Self :: Item > { match (self . a . next () , self . b . next ()) { (None , None) => None , (Some (a) , Some (b)) => Some ((a , b)) , (None , Some (_)) | (Some (_) , None) => { panic ! ("itertools: .zip_eq() reached end of one iterator before the other") } } } fn size_hint (& self) -> (usize , Option < usize >) { size_hint :: min (self . a . size_hint () , self . b . size_hint ()) } }
};
}
