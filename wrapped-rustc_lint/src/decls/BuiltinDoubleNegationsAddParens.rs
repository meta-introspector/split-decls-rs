macro_rules! BuiltinDoubleNegationsAddParens {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (lint_add_parens_suggestion , applicability = "maybe-incorrect")] pub (crate) struct BuiltinDoubleNegationsAddParens { # [suggestion_part (code = "(")] pub start_span : Span , # [suggestion_part (code = ")")] pub end_span : Span , }
    };
}

BuiltinDoubleNegationsAddParens!();