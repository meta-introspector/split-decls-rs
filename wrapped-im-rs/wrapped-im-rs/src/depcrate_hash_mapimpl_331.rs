// Generated macro for impl_331 (impl)
macro_rules! Depcrate_hash_mapimpl_331 {
() => {
// Module: crate::hash::map
// Provides: {"impl_331"}
// Dependencies: {}
impl < K , V , S > Add for HashMap < K , V , S > where K : Hash + Eq + Clone , V : Clone , S : BuildHasher , { type Output = HashMap < K , V , S > ; fn add (self , other : Self) -> Self :: Output { self . union (other) } }
};
}
