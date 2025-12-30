// Generated macro for check (function)
macro_rules! Depcrate_types_rc_mutexcheck {
() => {
// Module: crate::types::rc_mutex
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , hir_ty : & hir :: Ty < '_ > , qpath : & QPath < '_ > , def_id : DefId) -> bool { if cx . tcx . is_diagnostic_item (sym :: Rc , def_id) && let Some (arg) = qpath_generic_tys (qpath) . next () && arg . basic_res () . is_diag_item (cx , sym :: Mutex) { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , RC_MUTEX , hir_ty . span , "usage of `Rc<Mutex<_>>`" , | diag | { diag . help ("consider using `Rc<RefCell<_>>` or `Arc<Mutex<_>>` instead") ; }) ; return true ; } false }
};
}
