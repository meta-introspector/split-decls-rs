// Generated macro for VacantEntry (struct)
macro_rules! Depcrate_header_mapVacantEntry {
() => {
// Module: crate::header::map
// Provides: {"VacantEntry"}
// Dependencies: {}
# [doc = " A view into a single empty location in a `HeaderMap`."] # [doc = ""] # [doc = " This struct is returned as part of the `Entry` enum."] # [derive (Debug)] pub struct VacantEntry < 'a , T > { map : & 'a mut HeaderMap < T > , key : HeaderName , hash : HashValue , probe : usize , danger : bool , }
};
}
