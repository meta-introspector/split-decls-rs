// Generated macro for FormatUnknownTrait (struct)
macro_rules! Depcrate_errorsFormatUnknownTrait {
() => {
// Module: crate::errors
// Provides: {"FormatUnknownTrait"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (builtin_macros_format_unknown_trait)] # [note] pub (crate) struct FormatUnknownTrait < 'a > { # [primary_span] pub (crate) span : Span , pub (crate) ty : & 'a str , # [subdiagnostic] pub (crate) suggs : Vec < FormatUnknownTraitSugg > , }
};
}
