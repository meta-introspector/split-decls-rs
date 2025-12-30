// Generated macro for impl_537 (impl)
macro_rules! Depcrate_vectorimpl_537 {
() => {
// Module: crate::vector
// Provides: {"impl_537"}
// Dependencies: {}
impl < 's , 'a , A , OA > From < & 's Vector < & 'a A > > for Vector < OA > where A : ToOwned < Owned = OA > , OA : Borrow < A > + Clone , { fn from (vec : & Vector < & A >) -> Self { vec . iter () . map (| a | (* a) . to_owned ()) . collect () } }
};
}
