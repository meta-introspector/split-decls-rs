// Generated macro for InvalidAsmTemplateModifierConst (struct)
macro_rules! Depcrate_errorsInvalidAsmTemplateModifierConst {
() => {
// Module: crate::errors
// Provides: {"InvalidAsmTemplateModifierConst"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_lowering_invalid_asm_template_modifier_const)] pub (crate) struct InvalidAsmTemplateModifierConst { # [primary_span] # [label (ast_lowering_template_modifier)] pub placeholder_span : Span , # [label (ast_lowering_argument)] pub op_span : Span , }
};
}
