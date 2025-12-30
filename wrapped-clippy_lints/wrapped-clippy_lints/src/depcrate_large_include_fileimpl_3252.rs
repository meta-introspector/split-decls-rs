// Generated macro for impl_3252 (impl)
macro_rules! Depcrate_large_include_fileimpl_3252 {
() => {
// Module: crate::large_include_file
// Provides: {"impl_3252"}
// Dependencies: {}
impl LateLintPass < '_ > for LargeIncludeFile { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & '_ Expr < '_ >) { if let ExprKind :: Lit (lit) = & expr . kind && let len = match & lit . node { LitKind :: ByteStr (bstr , _) => bstr . as_byte_str () . len () , LitKind :: Str (sym , _) => sym . as_str () . len () , _ => return , } && len as u64 > self . max_file_size && let Some (macro_call) = root_macro_call_first_node (cx , expr) && let Some (macro_name) = cx . tcx . get_diagnostic_name (macro_call . def_id) && matches ! (macro_name , sym :: include_bytes_macro | sym :: include_str_macro) { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , LARGE_INCLUDE_FILE , expr . span . source_callsite () , "attempted to include a large file" , | diag | { diag . note (format ! ("the configuration allows a maximum size of {} bytes" , self . max_file_size)) ; } ,) ; } } }
};
}
