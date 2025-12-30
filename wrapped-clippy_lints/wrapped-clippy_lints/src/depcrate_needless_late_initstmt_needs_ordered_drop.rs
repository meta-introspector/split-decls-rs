// Generated macro for stmt_needs_ordered_drop (function)
macro_rules! Depcrate_needless_late_initstmt_needs_ordered_drop {
() => {
// Module: crate::needless_late_init
// Provides: {"stmt_needs_ordered_drop"}
// Dependencies: {}
fn stmt_needs_ordered_drop (cx : & LateContext < '_ > , stmt : & Stmt < '_ >) -> bool { let StmtKind :: Let (local) = stmt . kind else { return false ; } ; ! local . pat . walk_short (| pat | { if let PatKind :: Binding (.. , None) = pat . kind { ! needs_ordered_drop (cx , cx . typeck_results () . pat_ty (pat)) } else { true } }) }
};
}
