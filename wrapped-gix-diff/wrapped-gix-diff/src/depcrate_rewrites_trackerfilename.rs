// Generated macro for filename (function)
macro_rules! Depcrate_rewrites_trackerfilename {
() => {
// Module: crate::rewrites::tracker
// Provides: {"filename"}
// Dependencies: {}
fn filename (path : & BStr) -> & BStr { path . rfind_byte (b'/') . map_or (path , | idx | path [idx + 1 ..] . as_bstr ()) }
};
}
