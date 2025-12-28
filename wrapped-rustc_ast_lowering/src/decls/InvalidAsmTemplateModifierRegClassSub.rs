macro_rules! InvalidAsmTemplateModifierRegClassSub {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum InvalidAsmTemplateModifierRegClassSub { # [note (ast_lowering_support_modifiers)] SupportModifier { class_name : Symbol , modifiers : String } , # [note (ast_lowering_does_not_support_modifiers)] DoesNotSupportModifier { class_name : Symbol } , }
    };
}

InvalidAsmTemplateModifierRegClassSub!();