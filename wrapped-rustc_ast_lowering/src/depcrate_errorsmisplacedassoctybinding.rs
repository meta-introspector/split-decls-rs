// Generated macro for MisplacedAssocTyBinding (struct)
macro_rules! Depcrate_errorsMisplacedAssocTyBinding {
() => {
// Module: crate::errors
// Provides: {"MisplacedAssocTyBinding"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_lowering_assoc_ty_binding_in_dyn)] pub (crate) struct MisplacedAssocTyBinding { # [primary_span] pub span : Span , # [suggestion (code = " = impl" , applicability = "maybe-incorrect" , style = "verbose")] pub suggestion : Option < Span > , }
};
}
