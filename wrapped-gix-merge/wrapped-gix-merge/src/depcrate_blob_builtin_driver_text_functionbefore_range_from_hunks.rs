// Generated macro for before_range_from_hunks (function)
macro_rules! Depcrate_blob_builtin_driver_text_functionbefore_range_from_hunks {
() => {
// Module: crate::blob::builtin_driver::text::function
// Provides: {"before_range_from_hunks"}
// Dependencies: {}
fn before_range_from_hunks (hunks : & [Hunk]) -> Range < u32 > { hunks . first () . zip (hunks . last ()) . map (| (f , l) | f . before . start .. l . before . end) . expect ("at least one entry") }
};
}
