// Generated macro for TransparentNonZeroSized (struct)
macro_rules! Depcrate_errorsTransparentNonZeroSized {
() => {
// Module: crate::errors
// Provides: {"TransparentNonZeroSized"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_transparent_non_zero_sized , code = E0690)] pub (crate) struct TransparentNonZeroSized < 'a > { # [primary_span] # [label] pub span : Span , # [label (hir_analysis_labels)] pub spans : Vec < Span > , pub field_count : usize , pub desc : & 'a str , }
};
}
