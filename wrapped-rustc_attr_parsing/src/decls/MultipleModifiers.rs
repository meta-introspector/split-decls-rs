macro_rules! MultipleModifiers {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_multiple_modifiers)] pub (crate) struct MultipleModifiers { # [primary_span] pub span : Span , pub modifier : Symbol , }
    };
}

MultipleModifiers!()