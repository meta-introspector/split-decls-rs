// Generated macro for InvalidAsmTemplateModifierRegClass (struct)
macro_rules! Depcrate_errorsInvalidAsmTemplateModifierRegClass {
() => {
// Module: crate::errors
// Provides: {"InvalidAsmTemplateModifierRegClass"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_lowering_invalid_asm_template_modifier_reg_class)] pub (crate) struct InvalidAsmTemplateModifierRegClass { # [primary_span] # [label (ast_lowering_template_modifier)] pub placeholder_span : Span , # [label (ast_lowering_argument)] pub op_span : Span , # [subdiagnostic] pub sub : InvalidAsmTemplateModifierRegClassSub , }
};
}
