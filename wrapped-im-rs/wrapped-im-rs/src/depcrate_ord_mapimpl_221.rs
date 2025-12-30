// Generated macro for impl_221 (impl)
macro_rules! Depcrate_ord_mapimpl_221 {
() => {
// Module: crate::ord::map
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'a , K , V , OK , OV , RK , RV > From < & 'a collections :: HashMap < RK , RV > > for OrdMap < K , V > where K : Ord + Clone + From < OK > , V : Clone + From < OV > , OK : Borrow < RK > , OV : Borrow < RV > , RK : Hash + Eq + ToOwned < Owned = OK > , RV : ToOwned < Owned = OV > , { fn from (m : & 'a collections :: HashMap < RK , RV >) -> OrdMap < K , V > { m . iter () . map (| (k , v) | (k . to_owned () , v . to_owned ())) . collect () } }
};
}
