// Generated macro for impl_193 (impl)
macro_rules! Depcrate_ord_mapimpl_193 {
() => {
// Module: crate::ord::map
// Provides: {"impl_193"}
// Dependencies: {}
impl < K , V , RK , RV > Extend < (RK , RV) > for OrdMap < K , V > where K : Ord + Clone + From < RK > , V : Clone + From < RV > , { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = (RK , RV) > , { for (key , value) in iter { self . insert (From :: from (key) , From :: from (value)) ; } } }
};
}
