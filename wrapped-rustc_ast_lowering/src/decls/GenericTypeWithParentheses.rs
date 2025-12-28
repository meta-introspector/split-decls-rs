macro_rules! deps {
    () => {
        UseAngleBrackets!();
    };
}

macro_rules! GenericTypeWithParentheses {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (ast_lowering_generic_type_with_parentheses , code = E0214)] pub (crate) struct GenericTypeWithParentheses { # [primary_span] # [label] pub span : Span , # [subdiagnostic] pub sub : Option < UseAngleBrackets > , }
    };
}

GenericTypeWithParentheses!();