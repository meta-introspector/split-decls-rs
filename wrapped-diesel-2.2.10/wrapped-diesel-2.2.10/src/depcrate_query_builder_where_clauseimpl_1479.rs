// Generated macro for impl_1479 (impl)
macro_rules! Depcrate_query_builder_where_clauseimpl_1479 {
() => {
// Module: crate::query_builder::where_clause
// Provides: {"impl_1479"}
// Dependencies: {}
impl < 'a , DB , Predicate > From < WhereClause < Predicate > > for BoxedWhereClause < 'a , DB > where DB : Backend , Predicate : QueryFragment < DB > + Send + 'a , { fn from (where_clause : WhereClause < Predicate >) -> Self { BoxedWhereClause :: Where (Box :: new (where_clause . 0)) } }
};
}
