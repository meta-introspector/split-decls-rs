macro_rules! IllFormedAttributeInput {
    () => {
        # [derive (LintDiagnostic)] # [diag (attr_parsing_ill_formed_attribute_input)] pub (crate) struct IllFormedAttributeInput { pub num_suggestions : usize , pub suggestions : DiagArgValue , }
    };
}

IllFormedAttributeInput!()