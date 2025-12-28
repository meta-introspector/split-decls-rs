macro_rules! OverflowingLiteral {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_overflowing_literal)] # [note] pub (crate) struct OverflowingLiteral < 'a > { pub ty : & 'a str , pub lit : String , }
    };
}

OverflowingLiteral!()