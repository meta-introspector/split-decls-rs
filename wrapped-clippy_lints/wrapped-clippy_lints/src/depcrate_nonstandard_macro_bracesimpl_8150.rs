// Generated macro for impl_8150 (impl)
macro_rules! Depcrate_nonstandard_macro_bracesimpl_8150 {
() => {
// Module: crate::nonstandard_macro_braces
// Provides: {"impl_8150"}
// Dependencies: {}
impl EarlyLintPass for MacroBraces { fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & ast :: Item) { if let Some ((span , braces , snip)) = is_offending_macro (cx , item . span , self) { emit_help (cx , & snip , braces , span) ; self . done . insert (span) ; } } fn check_stmt (& mut self , cx : & EarlyContext < '_ > , stmt : & ast :: Stmt) { if let Some ((span , braces , snip)) = is_offending_macro (cx , stmt . span , self) { emit_help (cx , & snip , braces , span) ; self . done . insert (span) ; } } fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & ast :: Expr) { if let Some ((span , braces , snip)) = is_offending_macro (cx , expr . span , self) { emit_help (cx , & snip , braces , span) ; self . done . insert (span) ; } } fn check_ty (& mut self , cx : & EarlyContext < '_ > , ty : & ast :: Ty) { if let Some ((span , braces , snip)) = is_offending_macro (cx , ty . span , self) { emit_help (cx , & snip , braces , span) ; self . done . insert (span) ; } } }
};
}
