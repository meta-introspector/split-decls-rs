// Generated macro for OccupiedEntry (struct)
macro_rules! Depcrate_ord_mapOccupiedEntry {
() => {
// Module: crate::ord::map
// Provides: {"OccupiedEntry"}
// Dependencies: {}
# [doc = " An entry for a mapping that already exists in the map."] pub struct OccupiedEntry < 'a , K , V > where K : Ord + Clone , V : Clone , { map : & 'a mut OrdMap < K , V > , key : K , }
};
}
