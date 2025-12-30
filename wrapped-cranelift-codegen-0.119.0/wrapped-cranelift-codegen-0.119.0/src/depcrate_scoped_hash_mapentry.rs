// Generated macro for Entry (enum)
macro_rules! Depcrate_scoped_hash_mapEntry {
() => {
// Module: crate::scoped_hash_map
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A view into a single entry in a map, which may either be vacant or occupied."] # [doc = ""] # [doc = " This enum is constructed from the `entry` method on `ScopedHashMap`."] pub enum Entry < 'a , K : 'a , V : 'a > { Occupied (OccupiedEntry < 'a , K , V >) , Vacant (VacantEntry < 'a , K , V >) , }
};
}
