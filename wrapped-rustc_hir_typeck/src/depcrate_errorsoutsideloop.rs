// Generated macro for OutsideLoop (struct)
macro_rules! Depcrate_errorsOutsideLoop {
() => {
// Module: crate::errors
// Provides: {"OutsideLoop"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_outside_loop , code = E0268)] pub (crate) struct OutsideLoop < 'a > { # [primary_span] # [label] pub spans : Vec < Span > , pub name : & 'a str , pub is_break : bool , # [subdiagnostic] pub suggestion : Option < OutsideLoopSuggestion > , }
};
}
