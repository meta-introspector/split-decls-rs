// Generated macro for DeleteStatement (struct)
macro_rules! Depcrate_query_builder_delete_statementDeleteStatement {
() => {
// Module: crate::query_builder::delete_statement
// Provides: {"DeleteStatement"}
// Dependencies: {}
# [must_use = "Queries are only executed when calling `load`, `get_result` or similar."] # [doc = " Represents a SQL `DELETE` statement."] # [doc = ""] # [doc = " The type parameters on this struct represent:"] # [doc = ""] # [doc = " - `T`: The table we are deleting from."] # [doc = " - `U`: The `WHERE` clause of this query. The exact types used to represent"] # [doc = "   this are private, and you should not make any assumptions about them."] # [doc = " - `Ret`: The `RETURNING` clause of this query. The exact types used to"] # [doc = "   represent this are private. You can safely rely on the default type"] # [doc = "   representing the lack of a `RETURNING` clause."] pub struct DeleteStatement < T : QuerySource , U , Ret = NoReturningClause > { from_clause : FromClause < T > , where_clause : U , returning : Ret , }
};
}
