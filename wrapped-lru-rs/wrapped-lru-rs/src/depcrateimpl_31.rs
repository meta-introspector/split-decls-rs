// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl < K , V , S > Clone for LruCache < K , V , S > where K : Hash + PartialEq + Eq + Clone , V : Clone , S : BuildHasher + Clone , { fn clone (& self) -> Self { let map_cap = if self . is_unbounded () { self . len () } else { self . cap () . get () } ; let mut new_lru = LruCache :: construct (self . cap () , HashMap :: with_capacity_and_hasher (map_cap , self . map . hasher () . clone ()) ,) ; for (key , value) in self . iter () . rev () { new_lru . push (key . clone () , value . clone ()) ; } new_lru } }
};
}
