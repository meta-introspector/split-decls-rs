macro_rules! MalformedAttributeSub {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum MalformedAttributeSub { # [label (lint_bad_attribute_argument)] BadAttributeArgument (# [primary_span] Span) , # [label (lint_reason_must_be_string_literal)] ReasonMustBeStringLiteral (# [primary_span] Span) , # [label (lint_reason_must_come_last)] ReasonMustComeLast (# [primary_span] Span) , }
    };
}

MalformedAttributeSub!();