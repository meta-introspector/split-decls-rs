// Generated macro for impl_322 (impl)
macro_rules! Depcrate_hash_mapimpl_322 {
() => {
// Module: crate::hash::map
// Provides: {"impl_322"}
// Dependencies: {}
impl < K , V , S > Clone for HashMap < K , V , S > where K : Clone , V : Clone , { # [doc = " Clone a map."] # [doc = ""] # [doc = " Time: O(1)"] # [inline] fn clone (& self) -> Self { HashMap { root : self . root . clone () , pool : self . pool . clone () , size : self . size , hasher : self . hasher . clone () , } } }
};
}
