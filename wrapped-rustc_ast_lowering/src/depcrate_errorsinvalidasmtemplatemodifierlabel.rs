// Generated macro for InvalidAsmTemplateModifierLabel (struct)
macro_rules! Depcrate_errorsInvalidAsmTemplateModifierLabel {
() => {
// Module: crate::errors
// Provides: {"InvalidAsmTemplateModifierLabel"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_lowering_invalid_asm_template_modifier_label)] pub (crate) struct InvalidAsmTemplateModifierLabel { # [primary_span] # [label (ast_lowering_template_modifier)] pub placeholder_span : Span , # [label (ast_lowering_argument)] pub op_span : Span , }
};
}
