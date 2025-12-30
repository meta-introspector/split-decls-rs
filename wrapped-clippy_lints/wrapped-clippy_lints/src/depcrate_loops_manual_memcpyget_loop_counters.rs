// Generated macro for get_loop_counters (function)
macro_rules! Depcrate_loops_manual_memcpyget_loop_counters {
() => {
// Module: crate::loops::manual_memcpy
// Provides: {"get_loop_counters"}
// Dependencies: {}
fn get_loop_counters < 'a , 'tcx > (cx : & 'a LateContext < 'tcx > , body : & 'tcx Block < 'tcx > , expr : & 'tcx Expr < '_ > ,) -> Option < impl Iterator < Item = Start < 'tcx > > + 'a > { let mut increment_visitor = IncrementVisitor :: new (cx) ; walk_block (& mut increment_visitor , body) ; get_enclosing_block (cx , expr . hir_id) . and_then (| block | { increment_visitor . into_results () . filter_map (move | var_id | { let mut initialize_visitor = InitializeVisitor :: new (cx , expr , var_id) ; walk_block (& mut initialize_visitor , block) ; initialize_visitor . get_result () . map (| (_ , _ , initializer) | Start { id : var_id , kind : StartKind :: Counter { initializer } , }) }) . into () }) }
};
}
