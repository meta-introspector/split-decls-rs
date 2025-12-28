macro_rules! IdentifierNonAsciiChar {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_identifier_non_ascii_char)] pub (crate) struct IdentifierNonAsciiChar ;
    };
}

IdentifierNonAsciiChar!()