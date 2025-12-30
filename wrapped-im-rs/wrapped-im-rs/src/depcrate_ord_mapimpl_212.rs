// Generated macro for impl_212 (impl)
macro_rules! Depcrate_ord_mapimpl_212 {
() => {
// Module: crate::ord::map
// Provides: {"impl_212"}
// Dependencies: {}
impl < K , V , RK , RV > FromIterator < (RK , RV) > for OrdMap < K , V > where K : Ord + Clone + From < RK > , V : Clone + From < RV > , { fn from_iter < T > (i : T) -> Self where T : IntoIterator < Item = (RK , RV) > , { let mut m = OrdMap :: default () ; for (k , v) in i { m . insert (From :: from (k) , From :: from (v)) ; } m } }
};
}
