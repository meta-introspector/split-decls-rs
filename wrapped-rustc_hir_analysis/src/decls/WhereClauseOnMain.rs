macro_rules! WhereClauseOnMain {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_where_clause_on_main , code = E0646)] pub (crate) struct WhereClauseOnMain { # [primary_span] pub span : Span , # [label] pub generics_span : Option < Span > , }
    };
}

WhereClauseOnMain!()