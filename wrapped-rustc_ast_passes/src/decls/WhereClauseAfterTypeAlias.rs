macro_rules! WhereClauseAfterTypeAlias {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_where_clause_after_type_alias)] # [note] pub (crate) struct WhereClauseAfterTypeAlias { # [primary_span] pub span : Span , # [help] pub help : bool , }
    };
}

WhereClauseAfterTypeAlias!()