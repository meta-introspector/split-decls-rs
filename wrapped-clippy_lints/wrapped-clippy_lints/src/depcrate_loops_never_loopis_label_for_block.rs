// Generated macro for is_label_for_block (function)
macro_rules! Depcrate_loops_never_loopis_label_for_block {
() => {
// Module: crate::loops::never_loop
// Provides: {"is_label_for_block"}
// Dependencies: {}
fn is_label_for_block (cx : & LateContext < '_ > , dest : & Destination) -> bool { dest . target_id . is_ok_and (| hir_id | matches ! (cx . tcx . hir_node (hir_id) , Node :: Block (_))) }
};
}
