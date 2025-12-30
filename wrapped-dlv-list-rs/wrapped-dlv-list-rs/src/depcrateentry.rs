// Generated macro for Entry (enum)
macro_rules! DepcrateEntry {
() => {
// Module: crate
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " An entry in the list. This can be either occupied or vacant."] # [derive (Clone)] enum Entry < T > { # [doc = " An occupied entry contains actual entry data inserted by the user."] Occupied (OccupiedEntry < T >) , # [doc = " A vacant entry is one that can be reused."] Vacant (VacantEntry) , }
};
}
