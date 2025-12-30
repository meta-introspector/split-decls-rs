// Generated macro for impl_220 (impl)
macro_rules! Depcrate_ord_mapimpl_220 {
() => {
// Module: crate::ord::map
// Provides: {"impl_220"}
// Dependencies: {}
impl < K : Ord , V , RK : Eq + Hash , RV > From < collections :: HashMap < RK , RV > > for OrdMap < K , V > where K : Ord + Clone + From < RK > , V : Clone + From < RV > , { fn from (m : collections :: HashMap < RK , RV >) -> OrdMap < K , V > { m . into_iter () . collect () } }
};
}
