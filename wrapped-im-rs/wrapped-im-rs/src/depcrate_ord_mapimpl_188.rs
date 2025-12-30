// Generated macro for impl_188 (impl)
macro_rules! Depcrate_ord_mapimpl_188 {
() => {
// Module: crate::ord::map
// Provides: {"impl_188"}
// Dependencies: {}
impl < K , V > Hash for OrdMap < K , V > where K : Ord + Hash , V : Hash , { fn hash < H > (& self , state : & mut H) where H : Hasher , { for i in self . iter () { i . hash (state) ; } } }
};
}
