// Generated macro for render_const_eval_error (function)
macro_rules! Depcrate_interpretrender_const_eval_error {
() => {
// Module: crate::interpret
// Provides: {"render_const_eval_error"}
// Dependencies: {}
pub (crate) fn render_const_eval_error (db : & RootDatabase , e : ConstEvalError < '_ > , display_target : DisplayTarget ,) -> String { let span_formatter = | file_id , text_range : TextRange | { let source_root = db . file_source_root (file_id) . source_root_id (db) ; let source_root = db . source_root (source_root) . source_root (db) ; let path = source_root . path_for_file (& file_id) . map (| x | x . to_string ()) ; let path = path . as_deref () . unwrap_or ("<unknown file>") ; match db . line_index (file_id) . try_line_col (text_range . start ()) { Some (line_col) => format ! ("file://{path}:{}:{}" , line_col . line + 1 , line_col . col) , None => format ! ("file://{path} range {text_range:?}") , } } ; let mut r = String :: new () ; _ = e . pretty_print (& mut r , db , span_formatter , display_target) ; r }
};
}
