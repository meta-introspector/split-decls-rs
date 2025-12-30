// Generated macro for Entry (struct)
macro_rules! Depcrate_multi_index_writeEntry {
() => {
// Module: crate::multi_index::write
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " An entry suitable for sorting and writing"] pub (crate) struct Entry { pub (crate) id : gix_hash :: ObjectId , pub (crate) pack_index : u32 , pub (crate) pack_offset : crate :: data :: Offset , # [doc = " Used for sorting in case of duplicates"] index_mtime : SystemTime , }
};
}
