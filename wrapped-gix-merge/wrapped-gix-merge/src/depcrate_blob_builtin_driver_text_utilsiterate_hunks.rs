// Generated macro for iterate_hunks (function)
macro_rules! Depcrate_blob_builtin_driver_text_utilsiterate_hunks {
() => {
// Module: crate::blob::builtin_driver::text::utils
// Provides: {"iterate_hunks"}
// Dependencies: {}
# [doc = " Return an iterator over `(token_idx, hunk_idx, hunk_side)` from `hunks`."] fn iterate_hunks (hunks : & [Hunk]) -> impl Iterator < Item = (u32 , usize , Side) > + '_ { hunks . iter () . enumerate () . flat_map (| (hunk_idx , hunk) | { match hunk . side { Side :: Current | Side :: Other => & hunk . after , Side :: Ancestor => & hunk . before , } . clone () . map (move | idx | (idx , hunk_idx , hunk . side)) }) }
};
}
