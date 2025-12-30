// Generated macro for check (function)
macro_rules! Depcrate_loops_empty_loopcheck {
() => {
// Module: crate::loops::empty_loop
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , loop_block : & Block < '_ >) { let parent_hir_id = cx . tcx . parent_hir_id (expr . hir_id) ; if let Node :: Item (parent_node) = cx . tcx . hir_node (parent_hir_id) && matches ! (parent_node . kind , ItemKind :: Fn { .. }) && let attrs = cx . tcx . hir_attrs (parent_hir_id) && attrs . iter () . any (| attr | attr . has_name (sym :: rustc_intrinsic)) { return ; } if loop_block . stmts . is_empty () && loop_block . expr . is_none () && ! is_in_panic_handler (cx , expr) { let msg = "empty `loop {}` wastes CPU cycles" ; let help = if is_no_std_crate (cx) { "you should either use `panic!()` or add a call pausing or sleeping the thread to the loop body" } else { "you should either use `panic!()` or add `std::thread::sleep(..);` to the loop body" } ; span_lint_and_help (cx , EMPTY_LOOP , expr . span , msg , None , help) ; } }
};
}
