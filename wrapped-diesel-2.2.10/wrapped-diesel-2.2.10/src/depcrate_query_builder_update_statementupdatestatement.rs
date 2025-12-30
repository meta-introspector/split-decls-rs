// Generated macro for UpdateStatement (struct)
macro_rules! Depcrate_query_builder_update_statementUpdateStatement {
() => {
// Module: crate::query_builder::update_statement
// Provides: {"UpdateStatement"}
// Dependencies: {}
# [derive (Clone , Debug)] # [must_use = "Queries are only executed when calling `load`, `get_result` or similar."] # [doc = " Represents a complete `UPDATE` statement."] # [doc = ""] # [doc = " See [`update`](crate::update()) for usage examples, or [the update"] # [doc = " guide](https://diesel.rs/guides/all-about-updates/) for a more exhaustive"] # [doc = " set of examples."] pub struct UpdateStatement < T : QuerySource , U , V = SetNotCalled , Ret = NoReturningClause > { from_clause : T :: FromClause , where_clause : U , values : V , returning : Ret , }
};
}
