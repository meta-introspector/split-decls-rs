// Generated macro for OnConflictValues (struct)
macro_rules! Depcrate_query_builder_upsert_on_conflict_clauseOnConflictValues {
() => {
// Module: crate::query_builder::upsert::on_conflict_clause
// Provides: {"OnConflictValues"}
// Dependencies: {}
# [doc (hidden)] # [derive (Debug , Clone , Copy)] pub struct OnConflictValues < Values , Target , Action , WhereClause = NoWhereClause > { pub (crate) values : Values , pub (crate) target : Target , pub (crate) action : Action , # [doc = " Allow to apply filters on ON CONFLICT ... DO UPDATE ... WHERE ..."] pub (crate) where_clause : WhereClause , }
};
}
