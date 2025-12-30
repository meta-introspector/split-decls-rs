// Generated macro for push_backtrace (function)
macro_rules! Depcrate_executor_jsonpush_backtrace {
() => {
// Module: crate::executor::json
// Provides: {"push_backtrace"}
// Dependencies: {}
fn push_backtrace (errors : & mut Vec < Error > , expansion : & DiagnosticSpanMacroExpansion , file_name : & str ,) { if Path :: new (& expansion . span . file_name) == Path :: new (& file_name) { errors . push (Error { line_num : Some (expansion . span . line_start) , column_num : Some (expansion . span . column_start) , kind : ErrorKind :: Note , msg : format ! ("in this expansion of {}" , expansion . macro_decl_name) , require_annotation : true , }) ; } if let Some (previous_expansion) = & expansion . span . expansion { push_backtrace (errors , previous_expansion , file_name) ; } }
};
}
