// Generated macro for Entry (enum)
macro_rules! Depcrate_index_mapEntry {
() => {
// Module: crate::index_map
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A view into an entry in the map"] pub enum Entry < 'a , K , V , const N : usize > { # [doc = " The entry corresponding to the key `K` exists in the map"] Occupied (OccupiedEntry < 'a , K , V , N >) , # [doc = " The entry corresponding to the key `K` does not exist in the map"] Vacant (VacantEntry < 'a , K , V , N >) , }
};
}
