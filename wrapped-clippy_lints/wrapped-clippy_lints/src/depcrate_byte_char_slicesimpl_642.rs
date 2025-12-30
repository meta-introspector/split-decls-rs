// Generated macro for impl_642 (impl)
macro_rules! Depcrate_byte_char_slicesimpl_642 {
() => {
// Module: crate::byte_char_slices
// Provides: {"impl_642"}
// Dependencies: {}
impl EarlyLintPass for ByteCharSlice { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { if let Some (slice) = is_byte_char_slices (expr) && ! expr . span . from_expansion () { span_lint_and_sugg (cx , BYTE_CHAR_SLICES , expr . span , "can be more succinctly written as a byte str" , "try" , format ! ("b\"{slice}\"") , Applicability :: MachineApplicable ,) ; } } }
};
}
