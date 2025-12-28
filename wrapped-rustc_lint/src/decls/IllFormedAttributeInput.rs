macro_rules! IllFormedAttributeInput {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_ill_formed_attribute_input)] pub (crate) struct IllFormedAttributeInput { pub num_suggestions : usize , pub suggestions : DiagArgValue , # [note] pub has_docs : bool , pub docs : & 'static str , }
    };
}

IllFormedAttributeInput!();