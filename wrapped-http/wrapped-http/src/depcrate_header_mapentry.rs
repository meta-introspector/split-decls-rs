// Generated macro for Entry (enum)
macro_rules! Depcrate_header_mapEntry {
() => {
// Module: crate::header::map
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A view into a single location in a `HeaderMap`, which may be vacant or occupied."] # [derive (Debug)] pub enum Entry < 'a , T : 'a > { # [doc = " An occupied entry"] Occupied (OccupiedEntry < 'a , T >) , # [doc = " A vacant entry"] Vacant (VacantEntry < 'a , T >) , }
};
}
