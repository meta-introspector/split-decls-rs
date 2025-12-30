// Generated macro for impl_219 (impl)
macro_rules! Depcrate_ord_mapimpl_219 {
() => {
// Module: crate::ord::map
// Provides: {"impl_219"}
// Dependencies: {}
impl < 'a , K : Ord , V , RK , RV , OK , OV > From < & 'a Vec < (RK , RV) > > for OrdMap < K , V > where K : Ord + Clone + From < OK > , V : Clone + From < OV > , OK : Borrow < RK > , OV : Borrow < RV > , RK : ToOwned < Owned = OK > , RV : ToOwned < Owned = OV > , { fn from (m : & 'a Vec < (RK , RV) >) -> OrdMap < K , V > { m . iter () . map (| & (ref k , ref v) | (k . to_owned () , v . to_owned ())) . collect () } }
};
}
