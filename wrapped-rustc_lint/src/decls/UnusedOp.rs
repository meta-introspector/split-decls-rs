macro_rules! deps {
    () => {
        UnusedOpSuggestion!();
    };
}

macro_rules! UnusedOp {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_unused_op)] pub (crate) struct UnusedOp < 'a > { pub op : & 'a str , # [label] pub label : Span , # [subdiagnostic] pub suggestion : UnusedOpSuggestion , }
    };
}

UnusedOp!()