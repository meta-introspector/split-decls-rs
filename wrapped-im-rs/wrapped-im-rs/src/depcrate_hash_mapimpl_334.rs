// Generated macro for impl_334 (impl)
macro_rules! Depcrate_hash_mapimpl_334 {
() => {
// Module: crate::hash::map
// Provides: {"impl_334"}
// Dependencies: {}
impl < K , V , S , RK , RV > Extend < (RK , RV) > for HashMap < K , V , S > where K : Hash + Eq + Clone + From < RK > , V : Clone + From < RV > , S : BuildHasher , { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = (RK , RV) > , { for (key , value) in iter { self . insert (From :: from (key) , From :: from (value)) ; } } }
};
}
