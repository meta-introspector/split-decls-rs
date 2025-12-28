macro_rules! IllFormedAttributeInputLint {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_ill_formed_attribute_input)] pub (crate) struct IllFormedAttributeInputLint { # [primary_span] pub span : Span , pub num_suggestions : usize , pub suggestions : DiagArgValue , }
    };
}

IllFormedAttributeInputLint!();