// Generated macro for FindIter (struct)
macro_rules! Depcrate_packed_apiFindIter {
() => {
// Module: crate::packed::api
// Provides: {"FindIter"}
// Dependencies: {}
# [doc = " An iterator over non-overlapping matches from a packed searcher."] # [doc = ""] # [doc = " The lifetime `'s` refers to the lifetime of the underlying [`Searcher`],"] # [doc = " while the lifetime `'h` refers to the lifetime of the haystack being"] # [doc = " searched."] # [derive (Debug)] pub struct FindIter < 's , 'h > { searcher : & 's Searcher , haystack : & 'h [u8] , span : Span , }
};
}
