macro_rules! deps {
    () => {
        InvalidIssueStringCause!();
    };
}

macro_rules! InvalidIssueString {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (attr_parsing_invalid_issue_string , code = E0545)] pub (crate) struct InvalidIssueString { # [primary_span] pub span : Span , # [subdiagnostic] pub cause : Option < InvalidIssueStringCause > , }
    };
}

InvalidIssueString!();