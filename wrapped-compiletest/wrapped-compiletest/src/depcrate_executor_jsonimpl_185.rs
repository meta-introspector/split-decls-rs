// Generated macro for impl_185 (impl)
macro_rules! Depcrate_executor_jsonimpl_185 {
() => {
// Module: crate::executor::json
// Provides: {"impl_185"}
// Dependencies: {}
impl DiagnosticSpan { # [doc = " Returns the deepest source span in the macro call stack with a given file name."] # [doc = " This is either the supplied span, or the span for some macro callsite that expanded to it."] fn first_callsite_in_file (& self , file_name : & str) -> & DiagnosticSpan { if self . file_name == file_name { self } else { self . expansion . as_ref () . map (| origin | origin . span . first_callsite_in_file (file_name)) . unwrap_or (self) } } }
};
}
