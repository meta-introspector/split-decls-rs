// Generated macro for expr_local (function)
macro_rules! Depcrate_mirexpr_local {
() => {
// Module: crate::mir
// Provides: {"expr_local"}
// Dependencies: {}
# [doc = " Tries to determine the `Local` corresponding to `expr`, if any."] # [doc = " This function is expensive and should be used sparingly."] pub fn expr_local (tcx : TyCtxt < '_ > , expr : & Expr < '_ >) -> Option < Local > { enclosing_mir (tcx , expr . hir_id) . and_then (| mir | { mir . local_decls . iter_enumerated () . find_map (| (local , local_decl) | { if local_decl . source_info . span == expr . span { Some (local) } else { None } }) }) }
};
}
