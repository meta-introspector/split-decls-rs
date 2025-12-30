// Generated macro for emit_lint (function)
macro_rules! Depcrate_repeat_vec_with_capacityemit_lint {
() => {
// Module: crate::repeat_vec_with_capacity
// Provides: {"emit_lint"}
// Dependencies: {}
fn emit_lint (cx : & LateContext < '_ > , span : Span , kind : & str , note : & 'static str , sugg_msg : & 'static str , sugg : String) { span_lint_and_then (cx , REPEAT_VEC_WITH_CAPACITY , span , format ! ("repeating `Vec::with_capacity` using `{kind}`, which does not retain capacity") , | diag | { diag . note (note) ; diag . span_suggestion_verbose (span , sugg_msg , sugg , Applicability :: MaybeIncorrect) ; } ,) ; }
};
}
