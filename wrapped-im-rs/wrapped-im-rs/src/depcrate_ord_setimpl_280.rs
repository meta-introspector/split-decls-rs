// Generated macro for impl_280 (impl)
macro_rules! Depcrate_ord_setimpl_280 {
() => {
// Module: crate::ord::set
// Provides: {"impl_280"}
// Dependencies: {}
impl < 's , 'a , A , OA > From < & 's OrdSet < & 'a A > > for OrdSet < OA > where A : ToOwned < Owned = OA > + Ord + ? Sized , OA : Borrow < A > + Ord + Clone , { fn from (set : & OrdSet < & A >) -> Self { set . iter () . map (| a | (* a) . to_owned ()) . collect () } }
};
}
