// Generated macro for impl_1390 (impl)
macro_rules! Depcrate_query_builder_upsert_into_conflict_clauseimpl_1390 {
() => {
// Module: crate::query_builder::upsert::into_conflict_clause
// Provides: {"impl_1390"}
// Dependencies: {}
impl < 'a , ST , QS , DB , GB , Columns > IntoConflictValueClause for InsertFromSelect < BoxedSelectStatement < 'a , ST , QS , DB , GB > , Columns > { type ValueClause = InsertFromSelect < OnConflictSelectWrapper < BoxedSelectStatement < 'a , ST , QS , DB , GB > > , Columns , > ; fn into_value_clause (self) -> Self :: ValueClause { let InsertFromSelect { columns , query } = self ; InsertFromSelect { query : OnConflictSelectWrapper (query) , columns , } } }
};
}
