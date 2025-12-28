macro_rules! UnusedMultiple {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_unused_multiple)] pub (crate) struct UnusedMultiple { # [primary_span] # [suggestion (code = "" , applicability = "machine-applicable")] pub this : Span , # [note] pub other : Span , pub name : Symbol , }
    };
}

UnusedMultiple!()