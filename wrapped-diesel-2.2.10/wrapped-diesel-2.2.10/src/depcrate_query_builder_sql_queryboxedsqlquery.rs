// Generated macro for BoxedSqlQuery (struct)
macro_rules! Depcrate_query_builder_sql_queryBoxedSqlQuery {
() => {
// Module: crate::query_builder::sql_query
// Provides: {"BoxedSqlQuery"}
// Dependencies: {}
# [must_use = "Queries are only executed when calling `load`, `get_result`, or similar."] # [doc = " See [`SqlQuery::into_boxed`]."] # [doc = ""] # [doc = " [`SqlQuery::into_boxed`]: SqlQuery::into_boxed()"] # [allow (missing_debug_implementations)] pub struct BoxedSqlQuery < 'f , DB : Backend , Query > { query : Query , sql : String , binds : Vec < Box < dyn QueryFragment < DB > + Send + 'f > > , }
};
}
