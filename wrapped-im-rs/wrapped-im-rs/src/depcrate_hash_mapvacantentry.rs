// Generated macro for VacantEntry (struct)
macro_rules! Depcrate_hash_mapVacantEntry {
() => {
// Module: crate::hash::map
// Provides: {"VacantEntry"}
// Dependencies: {}
# [doc = " An entry for a mapping that does not already exist in the map."] pub struct VacantEntry < 'a , K , V , S > where K : Hash + Eq + Clone , V : Clone , S : BuildHasher , { map : & 'a mut HashMap < K , V , S > , hash : HashBits , key : K , }
};
}
