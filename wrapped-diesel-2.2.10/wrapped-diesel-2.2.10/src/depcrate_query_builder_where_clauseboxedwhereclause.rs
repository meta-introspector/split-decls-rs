// Generated macro for BoxedWhereClause (enum)
macro_rules! Depcrate_query_builder_where_clauseBoxedWhereClause {
() => {
// Module: crate::query_builder::where_clause
// Provides: {"BoxedWhereClause"}
// Dependencies: {}
# [allow (missing_debug_implementations)] pub enum BoxedWhereClause < 'a , DB > { Where (Box < dyn QueryFragment < DB > + Send + 'a >) , None , }
};
}
