// Generated macro for impl_1389 (impl)
macro_rules! Depcrate_query_builder_upsert_into_conflict_clauseimpl_1389 {
() => {
// Module: crate::query_builder::upsert::into_conflict_clause
// Provides: {"impl_1389"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC , Columns > IntoConflictValueClause for InsertFromSelect < SelectStatement < F , S , D , W , O , LOf , G , H , LC > , Columns > { type ValueClause = InsertFromSelect < OnConflictSelectWrapper < SelectStatement < F , S , D , W , O , LOf , G , H , LC > > , Columns , > ; fn into_value_clause (self) -> Self :: ValueClause { let InsertFromSelect { columns , query } = self ; InsertFromSelect { query : OnConflictSelectWrapper (query) , columns , } } }
};
}
