// Generated macro for impl_32 (impl)
macro_rules! Depcrate_iteratorimpl_32 {
() => {
// Module: crate::iterator
// Provides: {"impl_32"}
// Dependencies: {}
impl < L , R , A > Extend < A > for Either < L , R > where L : Extend < A > , R : Extend < A > , { fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = A > , { for_both ! (self , inner => inner . extend (iter)) } }
};
}
