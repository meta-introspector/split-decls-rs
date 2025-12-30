// Generated macro for impl_217 (impl)
macro_rules! Depcrate_ord_mapimpl_217 {
() => {
// Module: crate::ord::map
// Provides: {"impl_217"}
// Dependencies: {}
impl < 'a , K , V , RK , RV , OK , OV > From < & 'a [(RK , RV)] > for OrdMap < K , V > where K : Ord + Clone + From < OK > , V : Clone + From < OV > , OK : Borrow < RK > , OV : Borrow < RV > , RK : ToOwned < Owned = OK > , RV : ToOwned < Owned = OV > , { fn from (m : & 'a [(RK , RV)]) -> OrdMap < K , V > { m . iter () . map (| & (ref k , ref v) | (k . to_owned () , v . to_owned ())) . collect () } }
};
}
