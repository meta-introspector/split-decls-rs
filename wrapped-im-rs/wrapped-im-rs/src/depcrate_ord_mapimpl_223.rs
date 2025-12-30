// Generated macro for impl_223 (impl)
macro_rules! Depcrate_ord_mapimpl_223 {
() => {
// Module: crate::ord::map
// Provides: {"impl_223"}
// Dependencies: {}
impl < 'a , K : Ord , V , RK , RV , OK , OV > From < & 'a collections :: BTreeMap < RK , RV > > for OrdMap < K , V > where K : Ord + Clone + From < OK > , V : Clone + From < OV > , OK : Borrow < RK > , OV : Borrow < RV > , RK : Ord + ToOwned < Owned = OK > , RV : ToOwned < Owned = OV > , { fn from (m : & 'a collections :: BTreeMap < RK , RV >) -> OrdMap < K , V > { m . iter () . map (| (k , v) | (k . to_owned () , v . to_owned ())) . collect () } }
};
}
