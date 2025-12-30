// Generated macro for MultipleDefaults (struct)
macro_rules! Depcrate_errorsMultipleDefaults {
() => {
// Module: crate::errors
// Provides: {"MultipleDefaults"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (builtin_macros_multiple_defaults)] # [note] pub (crate) struct MultipleDefaults { # [primary_span] pub (crate) span : Span , # [label] pub (crate) first : Span , # [label (builtin_macros_additional)] pub additional : Vec < Span > , # [subdiagnostic] pub suggs : Vec < MultipleDefaultsSugg > , }
};
}
