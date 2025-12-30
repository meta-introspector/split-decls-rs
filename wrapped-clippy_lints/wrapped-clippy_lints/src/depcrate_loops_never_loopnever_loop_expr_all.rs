// Generated macro for never_loop_expr_all (function)
macro_rules! Depcrate_loops_never_loopnever_loop_expr_all {
() => {
// Module: crate::loops::never_loop
// Provides: {"never_loop_expr_all"}
// Dependencies: {}
fn never_loop_expr_all < 'tcx , T : Iterator < Item = & 'tcx Expr < 'tcx > > > (cx : & LateContext < 'tcx > , es : T , local_labels : & mut Vec < (HirId , bool) > , main_loop_id : HirId ,) -> NeverLoopResult { combine_seq_many (es . map (| e | never_loop_expr (cx , e , local_labels , main_loop_id))) }
};
}
