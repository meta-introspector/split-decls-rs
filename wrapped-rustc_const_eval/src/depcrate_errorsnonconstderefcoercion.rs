// Generated macro for NonConstDerefCoercion (struct)
macro_rules! Depcrate_errorsNonConstDerefCoercion {
() => {
// Module: crate::errors
// Provides: {"NonConstDerefCoercion"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (const_eval_non_const_deref_coercion , code = E0015)] # [note] pub struct NonConstDerefCoercion < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub kind : ConstContext , pub target_ty : Ty < 'tcx > , # [note (const_eval_target_note)] pub deref_target : Option < Span > , pub non_or_conditionally : & 'static str , }
};
}
