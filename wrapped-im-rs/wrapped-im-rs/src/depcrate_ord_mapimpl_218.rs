// Generated macro for impl_218 (impl)
macro_rules! Depcrate_ord_mapimpl_218 {
() => {
// Module: crate::ord::map
// Provides: {"impl_218"}
// Dependencies: {}
impl < K , V , RK , RV > From < Vec < (RK , RV) > > for OrdMap < K , V > where K : Ord + Clone + From < RK > , V : Clone + From < RV > , { fn from (m : Vec < (RK , RV) >) -> OrdMap < K , V > { m . into_iter () . collect () } }
};
}
