// Generated macro for impl_624 (impl)
macro_rules! Depcrate_rciter_implimpl_624 {
() => {
// Module: crate::rciter_impl
// Provides: {"impl_624"}
// Dependencies: {}
impl < A , I > Iterator for RcIter < I > where I : Iterator < Item = A > , { type Item = A ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . rciter . borrow_mut () . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . rciter . borrow () . size_hint () . 1) } }
};
}
