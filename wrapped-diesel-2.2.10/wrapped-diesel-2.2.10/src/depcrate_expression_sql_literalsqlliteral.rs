// Generated macro for SqlLiteral (struct)
macro_rules! Depcrate_expression_sql_literalSqlLiteral {
() => {
// Module: crate::expression::sql_literal
// Provides: {"SqlLiteral"}
// Dependencies: {}
# [derive (Debug , Clone , DieselNumericOps)] # [must_use = "Queries are only executed when calling `load`, `get_result`, or similar."] # [doc = " Returned by the [`sql()`] function."] # [doc = ""] # [doc = " [`sql()`]: crate::dsl::sql()"] pub struct SqlLiteral < ST , T = self :: private :: Empty > { sql : String , inner : T , _marker : PhantomData < ST > , }
};
}
