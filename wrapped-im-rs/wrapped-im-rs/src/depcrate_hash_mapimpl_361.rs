// Generated macro for impl_361 (impl)
macro_rules! Depcrate_hash_mapimpl_361 {
() => {
// Module: crate::hash::map
// Provides: {"impl_361"}
// Dependencies: {}
impl < K , V , S > IntoIterator for HashMap < K , V , S > where K : Hash + Eq + Clone , V : Clone , S : BuildHasher , { type Item = (K , V) ; type IntoIter = ConsumingIter < (K , V) > ; # [inline] fn into_iter (self) -> Self :: IntoIter { ConsumingIter { it : NodeDrain :: new (& self . pool . 0 , self . root , self . size) , } } }
};
}
