// Generated macro for Entry (enum)
macro_rules! Depcrate_mapEntry {
() => {
// Module: crate::map
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " An entry in a `LiteMap`, which may be either occupied or vacant."] # [allow (clippy :: exhaustive_enums)] pub enum Entry < 'a , K , V , S > { Occupied (OccupiedEntry < 'a , K , V , S >) , Vacant (VacantEntry < 'a , K , V , S >) , }
};
}
