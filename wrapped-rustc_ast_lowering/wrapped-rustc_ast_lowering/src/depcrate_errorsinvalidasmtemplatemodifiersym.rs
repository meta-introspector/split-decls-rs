// Generated macro for InvalidAsmTemplateModifierSym (struct)
macro_rules! Depcrate_errorsInvalidAsmTemplateModifierSym {
() => {
// Module: crate::errors
// Provides: {"InvalidAsmTemplateModifierSym"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_lowering_invalid_asm_template_modifier_sym)] pub (crate) struct InvalidAsmTemplateModifierSym { # [primary_span] # [label (ast_lowering_template_modifier)] pub placeholder_span : Span , # [label (ast_lowering_argument)] pub op_span : Span , }
};
}
