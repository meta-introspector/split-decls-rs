// Generated macro for never_loop_block (function)
macro_rules! Depcrate_loops_never_loopnever_loop_block {
() => {
// Module: crate::loops::never_loop
// Provides: {"never_loop_block"}
// Dependencies: {}
fn never_loop_block < 'tcx > (cx : & LateContext < 'tcx > , block : & Block < 'tcx > , local_labels : & mut Vec < (HirId , bool) > , main_loop_id : HirId ,) -> NeverLoopResult { let iter = block . stmts . iter () . filter_map (stmt_to_expr) . chain (block . expr . map (| expr | (expr , None))) ; combine_seq_many (iter . map (| (e , els) | { let e = never_loop_expr (cx , e , local_labels , main_loop_id) ; els . map_or (e . clone () , | els | { combine_seq (e , | | match never_loop_block (cx , els , local_labels , main_loop_id) { NeverLoopResult :: MayContinueMainLoop => NeverLoopResult :: MayContinueMainLoop , NeverLoopResult :: Diverging { .. } | NeverLoopResult :: Normal => NeverLoopResult :: Normal , }) }) })) }
};
}
