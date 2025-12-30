// Generated macro for potentially_bound_to_mutable_ref (function)
macro_rules! Depcrate_borrow_deref_refpotentially_bound_to_mutable_ref {
() => {
// Module: crate::borrow_deref_ref
// Provides: {"potentially_bound_to_mutable_ref"}
// Dependencies: {}
# [doc = " Checks if `expr` is used as part of a `let` statement containing a `ref mut` binding."] fn potentially_bound_to_mutable_ref < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) -> bool { matches ! (cx . tcx . parent_hir_node (expr . hir_id) , Node :: LetStmt (let_stmt) if let_stmt . pat . contains_explicit_ref_binding () == Some (Mutability :: Mut)) }
};
}
