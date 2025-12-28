macro_rules! deps {
    () => {
        AssocTyParenthesesSub!();
    };
}

macro_rules! AssocTyParentheses {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (ast_lowering_assoc_ty_parentheses)] pub (crate) struct AssocTyParentheses { # [primary_span] pub span : Span , # [subdiagnostic] pub sub : AssocTyParenthesesSub , }
    };
}

AssocTyParentheses!()