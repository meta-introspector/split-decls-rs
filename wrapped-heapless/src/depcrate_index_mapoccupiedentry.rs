// Generated macro for OccupiedEntry (struct)
macro_rules! Depcrate_index_mapOccupiedEntry {
() => {
// Module: crate::index_map
// Provides: {"OccupiedEntry"}
// Dependencies: {}
# [doc = " An occupied entry which can be manipulated"] pub struct OccupiedEntry < 'a , K , V , const N : usize > { key : K , probe : usize , pos : usize , core : & 'a mut CoreMap < K , V , N > , }
};
}
