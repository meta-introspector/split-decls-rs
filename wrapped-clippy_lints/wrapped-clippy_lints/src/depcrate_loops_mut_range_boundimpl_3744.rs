// Generated macro for impl_3744 (impl)
macro_rules! Depcrate_loops_mut_range_boundimpl_3744 {
() => {
// Module: crate::loops::mut_range_bound
// Provides: {"impl_3744"}
// Dependencies: {}
impl BreakAfterExprVisitor { pub fn is_found (cx : & LateContext < '_ > , hir_id : HirId) -> bool { let mut visitor = BreakAfterExprVisitor { hir_id , past_expr : false , break_after_expr : false , } ; get_enclosing_block (cx , hir_id) . is_some_and (| block | { let _ = visitor . visit_block (block) ; visitor . break_after_expr }) } }
};
}
