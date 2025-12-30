// Generated macro for select (function)
macro_rules! Depcrate_query_builder_functionsselect {
() => {
// Module: crate::query_builder::functions
// Provides: {"select"}
// Dependencies: {}
# [doc = " Creates a bare select statement, with no from clause. Primarily used for"] # [doc = " testing diesel itself, but likely useful for third party crates as well. The"] # [doc = " given expressions must be selectable from anywhere."] pub fn select < T > (expression : T) -> crate :: dsl :: select < T > where T : Expression , crate :: dsl :: select < T > : AsQuery , { SelectStatement :: new (SelectClause (expression) , super :: NoFromClause , NoDistinctClause , super :: where_clause :: NoWhereClause , super :: order_clause :: NoOrderClause , super :: limit_offset_clause :: LimitOffsetClause { limit_clause : super :: limit_clause :: NoLimitClause , offset_clause : super :: offset_clause :: NoOffsetClause , } , super :: group_by_clause :: NoGroupByClause , super :: having_clause :: NoHavingClause , super :: locking_clause :: NoLockingClause ,) }
};
}
