// Generated macro for OccupiedEntry (struct)
macro_rules! Depcrate_hash_mapOccupiedEntry {
() => {
// Module: crate::hash::map
// Provides: {"OccupiedEntry"}
// Dependencies: {}
# [doc = " An entry for a mapping that already exists in the map."] pub struct OccupiedEntry < 'a , K , V , S > where K : Hash + Eq + Clone , V : Clone , S : BuildHasher , { map : & 'a mut HashMap < K , V , S > , hash : HashBits , key : K , }
};
}
