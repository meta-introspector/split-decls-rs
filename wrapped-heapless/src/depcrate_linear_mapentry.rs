// Generated macro for Entry (enum)
macro_rules! Depcrate_linear_mapEntry {
() => {
// Module: crate::linear_map
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A view into an entry in the map"] pub enum Entry < 'a , K , V > { # [doc = " The entry corresponding to the key `K` exists in the map"] Occupied (OccupiedEntry < 'a , K , V >) , # [doc = " The entry corresponding to the key `K` does not exist in the map"] Vacant (VacantEntry < 'a , K , V >) , }
};
}
