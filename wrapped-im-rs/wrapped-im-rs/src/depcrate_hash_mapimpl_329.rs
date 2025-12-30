// Generated macro for impl_329 (impl)
macro_rules! Depcrate_hash_mapimpl_329 {
() => {
// Module: crate::hash::map
// Provides: {"impl_329"}
// Dependencies: {}
impl < K , V , S > Hash for HashMap < K , V , S > where K : Hash + Eq , V : Hash , S : BuildHasher , { fn hash < H > (& self , state : & mut H) where H : Hasher , { for i in self . iter () { i . hash (state) ; } } }
};
}
