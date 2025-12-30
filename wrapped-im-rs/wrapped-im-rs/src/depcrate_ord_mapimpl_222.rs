// Generated macro for impl_222 (impl)
macro_rules! Depcrate_ord_mapimpl_222 {
() => {
// Module: crate::ord::map
// Provides: {"impl_222"}
// Dependencies: {}
impl < K : Ord , V , RK , RV > From < collections :: BTreeMap < RK , RV > > for OrdMap < K , V > where K : Ord + Clone + From < RK > , V : Clone + From < RV > , { fn from (m : collections :: BTreeMap < RK , RV >) -> OrdMap < K , V > { m . into_iter () . collect () } }
};
}
