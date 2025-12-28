macro_rules! TooLargeCharCast {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_too_large_char_cast)] # [note] pub (crate) struct TooLargeCharCast { pub literal : u128 , }
    };
}

TooLargeCharCast!()