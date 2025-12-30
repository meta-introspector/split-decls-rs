// Generated macro for mut_warn_with_span (function)
macro_rules! Depcrate_loops_mut_range_boundmut_warn_with_span {
() => {
// Module: crate::loops::mut_range_bound
// Provides: {"mut_warn_with_span"}
// Dependencies: {}
fn mut_warn_with_span (cx : & LateContext < '_ > , span : Option < Span >) { if let Some (sp) = span { span_lint_and_note (cx , MUT_RANGE_BOUND , sp , "attempt to mutate range bound within loop" , None , "the range of the loop is unchanged" ,) ; } }
};
}
