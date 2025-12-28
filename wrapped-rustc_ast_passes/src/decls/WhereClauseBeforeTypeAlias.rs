macro_rules! deps {
    () => {
        WhereClauseBeforeTypeAliasSugg!();
    };
}

macro_rules! WhereClauseBeforeTypeAlias {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (ast_passes_where_clause_before_type_alias)] # [note] pub (crate) struct WhereClauseBeforeTypeAlias { # [primary_span] pub span : Span , # [subdiagnostic] pub sugg : WhereClauseBeforeTypeAliasSugg , }
    };
}

WhereClauseBeforeTypeAlias!()