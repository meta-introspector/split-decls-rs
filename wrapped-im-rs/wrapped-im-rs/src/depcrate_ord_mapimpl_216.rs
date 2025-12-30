// Generated macro for impl_216 (impl)
macro_rules! Depcrate_ord_mapimpl_216 {
() => {
// Module: crate::ord::map
// Provides: {"impl_216"}
// Dependencies: {}
impl < 'm , 'k , 'v , K , V , OK , OV > From < & 'm OrdMap < & 'k K , & 'v V > > for OrdMap < OK , OV > where K : Ord + ToOwned < Owned = OK > + ? Sized , V : ToOwned < Owned = OV > + ? Sized , OK : Ord + Clone + Borrow < K > , OV : Clone + Borrow < V > , { fn from (m : & OrdMap < & K , & V >) -> Self { m . iter () . map (| (k , v) | ((* k) . to_owned () , (* v) . to_owned ())) . collect () } }
};
}
