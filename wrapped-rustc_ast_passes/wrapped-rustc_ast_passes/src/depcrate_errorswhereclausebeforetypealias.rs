// Generated macro for WhereClauseBeforeTypeAlias (struct)
macro_rules! Depcrate_errorsWhereClauseBeforeTypeAlias {
() => {
// Module: crate::errors
// Provides: {"WhereClauseBeforeTypeAlias"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_where_clause_before_type_alias)] # [note] pub (crate) struct WhereClauseBeforeTypeAlias { # [primary_span] pub span : Span , # [subdiagnostic] pub sugg : WhereClauseBeforeTypeAliasSugg , }
};
}
