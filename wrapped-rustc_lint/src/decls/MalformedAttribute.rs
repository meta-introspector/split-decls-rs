macro_rules! deps {
    () => {
        MalformedAttributeSub!();
    };
}

macro_rules! MalformedAttribute {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (lint_malformed_attribute , code = E0452)] pub (crate) struct MalformedAttribute { # [primary_span] pub span : Span , # [subdiagnostic] pub sub : MalformedAttributeSub , }
    };
}

MalformedAttribute!();