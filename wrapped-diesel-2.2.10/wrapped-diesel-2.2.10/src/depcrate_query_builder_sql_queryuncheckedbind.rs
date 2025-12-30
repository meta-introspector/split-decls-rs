// Generated macro for UncheckedBind (struct)
macro_rules! Depcrate_query_builder_sql_queryUncheckedBind {
() => {
// Module: crate::query_builder::sql_query
// Provides: {"UncheckedBind"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [must_use = "Queries are only executed when calling `load`, `get_result` or similar."] # [doc = " Returned by the [`SqlQuery::bind()`] method when binding a value to a fragment of SQL."] # [doc = ""] pub struct UncheckedBind < Query , Value , ST > { query : Query , value : Value , _marker : PhantomData < ST > , }
};
}
