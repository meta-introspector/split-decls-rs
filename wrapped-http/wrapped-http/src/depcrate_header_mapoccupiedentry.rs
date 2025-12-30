// Generated macro for OccupiedEntry (struct)
macro_rules! Depcrate_header_mapOccupiedEntry {
() => {
// Module: crate::header::map
// Provides: {"OccupiedEntry"}
// Dependencies: {}
# [doc = " A view into a single occupied location in a `HeaderMap`."] # [doc = ""] # [doc = " This struct is returned as part of the `Entry` enum."] # [derive (Debug)] pub struct OccupiedEntry < 'a , T > { map : & 'a mut HeaderMap < T > , probe : usize , index : usize , }
};
}
