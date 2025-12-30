// Generated macro for impl_264 (impl)
macro_rules! Depcrate_ord_setimpl_264 {
() => {
// Module: crate::ord::set
// Provides: {"impl_264"}
// Dependencies: {}
impl < A , R > Extend < R > for OrdSet < A > where A : Ord + Clone + From < R > , { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = R > , { for value in iter { self . insert (From :: from (value)) ; } } }
};
}
