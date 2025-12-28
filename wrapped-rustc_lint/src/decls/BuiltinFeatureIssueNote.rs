macro_rules! BuiltinFeatureIssueNote {
    () => {
        # [derive (Subdiagnostic)] # [note (lint_note)] pub (crate) struct BuiltinFeatureIssueNote { pub n : NonZero < u32 > , }
    };
}

BuiltinFeatureIssueNote!()