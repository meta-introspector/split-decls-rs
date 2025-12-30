// Generated macro for SqlQuery (struct)
macro_rules! Depcrate_query_builder_sql_querySqlQuery {
() => {
// Module: crate::query_builder::sql_query
// Provides: {"SqlQuery"}
// Dependencies: {}
# [derive (Debug , Clone)] # [must_use = "Queries are only executed when calling `load`, `get_result` or similar."] # [doc = " The return value of `sql_query`."] # [doc = ""] # [doc = " Unlike most queries in Diesel, `SqlQuery` loads its data by column name,"] # [doc = " rather than by index. This means that you cannot deserialize this query into"] # [doc = " a tuple, and any structs used must implement `QueryableByName`."] # [doc = ""] # [doc = " See [`sql_query`](crate::sql_query()) for examples."] pub struct SqlQuery < Inner = self :: private :: Empty > { inner : Inner , query : String , }
};
}
