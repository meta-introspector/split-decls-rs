// Generated macro for line_index (function)
macro_rules! Depcrateline_index {
() => {
// Module: crate
// Provides: {"line_index"}
// Dependencies: {}
fn line_index (db : & dyn LineIndexDatabase , file_id : FileId) -> Arc < LineIndex > { let text = db . file_text (file_id) . text (db) ; Arc :: new (LineIndex :: new (text)) }
};
}
