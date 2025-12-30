// Generated macro for NonConstClosureNote (enum)
macro_rules! Depcrate_errorsNonConstClosureNote {
() => {
// Module: crate::errors
// Provides: {"NonConstClosureNote"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub enum NonConstClosureNote { # [note (const_eval_closure_fndef_not_const)] FnDef { # [primary_span] span : Span , } , # [note (const_eval_fn_ptr_call)] FnPtr , # [note (const_eval_closure_call)] Closure , }
};
}
