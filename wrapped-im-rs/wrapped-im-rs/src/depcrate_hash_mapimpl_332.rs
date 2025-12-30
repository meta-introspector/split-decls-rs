// Generated macro for impl_332 (impl)
macro_rules! Depcrate_hash_mapimpl_332 {
() => {
// Module: crate::hash::map
// Provides: {"impl_332"}
// Dependencies: {}
impl < 'a , K , V , S > Add for & 'a HashMap < K , V , S > where K : Hash + Eq + Clone , V : Clone , S : BuildHasher , { type Output = HashMap < K , V , S > ; fn add (self , other : Self) -> Self :: Output { self . clone () . union (other . clone ()) } }
};
}
