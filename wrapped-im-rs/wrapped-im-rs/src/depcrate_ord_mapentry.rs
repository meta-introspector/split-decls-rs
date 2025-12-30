// Generated macro for Entry (enum)
macro_rules! Depcrate_ord_mapEntry {
() => {
// Module: crate::ord::map
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A handle for a key and its associated value."] pub enum Entry < 'a , K , V > where K : Ord + Clone , V : Clone , { # [doc = " An entry which exists in the map."] Occupied (OccupiedEntry < 'a , K , V >) , # [doc = " An entry which doesn't exist in the map."] Vacant (VacantEntry < 'a , K , V >) , }
};
}
