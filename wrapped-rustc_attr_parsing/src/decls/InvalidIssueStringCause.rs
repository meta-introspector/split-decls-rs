macro_rules! InvalidIssueStringCause {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum InvalidIssueStringCause { # [label (attr_parsing_must_not_be_zero)] MustNotBeZero { # [primary_span] span : Span , } , # [label (attr_parsing_empty)] Empty { # [primary_span] span : Span , } , # [label (attr_parsing_invalid_digit)] InvalidDigit { # [primary_span] span : Span , } , # [label (attr_parsing_pos_overflow)] PosOverflow { # [primary_span] span : Span , } , # [label (attr_parsing_neg_overflow)] NegOverflow { # [primary_span] span : Span , } , }
    };
}

InvalidIssueStringCause!();