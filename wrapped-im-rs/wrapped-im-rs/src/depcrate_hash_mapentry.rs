// Generated macro for Entry (enum)
macro_rules! Depcrate_hash_mapEntry {
() => {
// Module: crate::hash::map
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A handle for a key and its associated value."] # [doc = ""] # [doc = " ## Performance Note"] # [doc = ""] # [doc = " When using an `Entry`, the key is only ever hashed once, when you"] # [doc = " create the `Entry`. Operations on an `Entry` will never trigger a"] # [doc = " rehash, where eg. a `contains_key(key)` followed by an"] # [doc = " `insert(key, default_value)` (the equivalent of"] # [doc = " `Entry::or_insert()`) would need to hash the key once for the"] # [doc = " `contains_key` and again for the `insert`. The operations"] # [doc = " generally perform similarly otherwise."] pub enum Entry < 'a , K , V , S > where K : Hash + Eq + Clone , V : Clone , S : BuildHasher , { # [doc = " An entry which exists in the map."] Occupied (OccupiedEntry < 'a , K , V , S >) , # [doc = " An entry which doesn't exist in the map."] Vacant (VacantEntry < 'a , K , V , S >) , }
};
}
