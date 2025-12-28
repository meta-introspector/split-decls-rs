macro_rules! deps {
    () => {
        AssociatedSuggestion!();
        AssociatedSuggestion2!();
    };
}

macro_rules! EqualityInWhere {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (ast_passes_equality_in_where)] # [note] pub (crate) struct EqualityInWhere { # [primary_span] # [label] pub span : Span , # [subdiagnostic] pub assoc : Option < AssociatedSuggestion > , # [subdiagnostic] pub assoc2 : Option < AssociatedSuggestion2 > , }
    };
}

EqualityInWhere!();