// Generated macro for impl_1366 (impl)
macro_rules! Depcrate_query_builder_update_statementimpl_1366 {
() => {
// Module: crate::query_builder::update_statement
// Provides: {"impl_1366"}
// Dependencies: {}
impl < T : QuerySource , U > UpdateStatement < T , U , SetNotCalled > { pub (crate) fn new (target : UpdateTarget < T , U >) -> Self { UpdateStatement { from_clause : target . table . from_clause () , where_clause : target . where_clause , values : SetNotCalled , returning : NoReturningClause , } } # [doc = " Provides the `SET` clause of the `UPDATE` statement."] # [doc = ""] # [doc = " See [`update`](crate::update()) for usage examples, or [the update"] # [doc = " guide](https://diesel.rs/guides/all-about-updates/) for a more exhaustive"] # [doc = " set of examples."] pub fn set < V > (self , values : V) -> UpdateStatement < T , U , V :: Changeset > where T : Table , V : changeset :: AsChangeset < Target = T > , UpdateStatement < T , U , V :: Changeset > : AsQuery , { UpdateStatement { from_clause : self . from_clause , where_clause : self . where_clause , values : values . as_changeset () , returning : self . returning , } } }
};
}
