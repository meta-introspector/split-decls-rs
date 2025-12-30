// Generated macro for range_by_side (function)
macro_rules! Depcrate_blob_builtin_driver_text_utilsrange_by_side {
() => {
// Module: crate::blob::builtin_driver::text::utils
// Provides: {"range_by_side"}
// Dependencies: {}
fn range_by_side (hunk : & mut Hunk) -> & mut Range < u32 > { match hunk . side { Side :: Current | Side :: Other => & mut hunk . after , Side :: Ancestor => & mut hunk . before , } }
};
}
