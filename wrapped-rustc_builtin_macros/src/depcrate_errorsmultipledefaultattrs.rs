// Generated macro for MultipleDefaultAttrs (struct)
macro_rules! Depcrate_errorsMultipleDefaultAttrs {
() => {
// Module: crate::errors
// Provides: {"MultipleDefaultAttrs"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (builtin_macros_multiple_default_attrs)] # [note] pub (crate) struct MultipleDefaultAttrs { # [primary_span] pub (crate) span : Span , # [label] pub (crate) first : Span , # [label (builtin_macros_label_again)] pub (crate) first_rest : Span , # [help] pub (crate) rest : MultiSpan , pub (crate) only_one : bool , # [subdiagnostic] pub (crate) sugg : MultipleDefaultAttrsSugg , }
};
}
