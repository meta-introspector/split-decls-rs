// Generated macro for filename (function)
macro_rules! Depcrate_tree_editorfilename {
() => {
// Module: crate::tree::editor
// Provides: {"filename"}
// Dependencies: {}
fn filename (path : & BStr) -> & BStr { path . rfind_byte (b'/') . map_or (path , | pos | & path [pos + 1 ..]) }
};
}
