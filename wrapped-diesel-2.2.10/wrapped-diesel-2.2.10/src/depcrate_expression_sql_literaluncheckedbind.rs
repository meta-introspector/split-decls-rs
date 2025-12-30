// Generated macro for UncheckedBind (struct)
macro_rules! Depcrate_expression_sql_literalUncheckedBind {
() => {
// Module: crate::expression::sql_literal
// Provides: {"UncheckedBind"}
// Dependencies: {}
# [derive (QueryId , Debug , Clone , Copy)] # [must_use = "Queries are only executed when calling `load`, `get_result`, or similar."] # [doc = " Returned by the [`SqlLiteral::bind()`] method when binding a value to a fragment of SQL."] # [doc = ""] pub struct UncheckedBind < Query , Value > { query : Query , value : Value , }
};
}
