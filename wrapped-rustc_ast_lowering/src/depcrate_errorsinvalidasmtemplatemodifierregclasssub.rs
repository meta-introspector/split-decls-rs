// Generated macro for InvalidAsmTemplateModifierRegClassSub (enum)
macro_rules! Depcrate_errorsInvalidAsmTemplateModifierRegClassSub {
() => {
// Module: crate::errors
// Provides: {"InvalidAsmTemplateModifierRegClassSub"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum InvalidAsmTemplateModifierRegClassSub { # [note (ast_lowering_support_modifiers)] SupportModifier { class_name : Symbol , modifiers : String } , # [note (ast_lowering_does_not_support_modifiers)] DoesNotSupportModifier { class_name : Symbol } , }
};
}
