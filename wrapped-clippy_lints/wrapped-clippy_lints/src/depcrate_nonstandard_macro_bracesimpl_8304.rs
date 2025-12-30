// Generated macro for impl_8304 (impl)
macro_rules! Depcrate_nonstandard_macro_bracesimpl_8304 {
() => {
// Module: crate::nonstandard_macro_braces
// Provides: {"impl_8304"}
// Dependencies: {}
impl EarlyLintPass for MacroBraces { fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & ast :: Item) { if let Some (MacroInfo { callsite_span , callsite_snippet , braces , .. }) = is_offending_macro (cx , item . span , self) { emit_help (cx , & callsite_snippet , braces , callsite_span , false) ; self . done . insert (callsite_span) ; } } fn check_stmt (& mut self , cx : & EarlyContext < '_ > , stmt : & ast :: Stmt) { if let Some (MacroInfo { callsite_span , callsite_snippet , braces , old_open_brace , }) = is_offending_macro (cx , stmt . span , self) { let add_semi = matches ! (stmt . kind , ast :: StmtKind :: Expr (..)) && old_open_brace == '{' ; emit_help (cx , & callsite_snippet , braces , callsite_span , add_semi) ; self . done . insert (callsite_span) ; } } fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & ast :: Expr) { if let Some (MacroInfo { callsite_span , callsite_snippet , braces , .. }) = is_offending_macro (cx , expr . span , self) { emit_help (cx , & callsite_snippet , braces , callsite_span , false) ; self . done . insert (callsite_span) ; } } fn check_ty (& mut self , cx : & EarlyContext < '_ > , ty : & ast :: Ty) { if let Some (MacroInfo { callsite_span , braces , callsite_snippet , .. }) = is_offending_macro (cx , ty . span , self) { emit_help (cx , & callsite_snippet , braces , callsite_span , false) ; self . done . insert (callsite_span) ; } } }
};
}
