macro_rules! SurrogateCharCast {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_surrogate_char_cast)] # [note] pub (crate) struct SurrogateCharCast { pub literal : u128 , }
    };
}

SurrogateCharCast!();