// Generated macro for emit_note (function)
macro_rules! Depcrate_methods_swap_with_temporaryemit_note {
() => {
// Module: crate::methods::swap_with_temporary
// Provides: {"emit_note"}
// Dependencies: {}
fn emit_note (diag : & mut Diag < '_ , () > , base : & Expr < '_ > , expr : & Expr < '_ > , expr_temp : & Expr < '_ >) -> bool { if base . span . eq_ctxt (expr . span) { diag . span_note (expr_temp . span . source_callsite () , MSG_TEMPORARY) ; true } else { diag . span_note (expr . span . source_callsite () , MSG_TEMPORARY_REFMUT) ; false } }
};
}
