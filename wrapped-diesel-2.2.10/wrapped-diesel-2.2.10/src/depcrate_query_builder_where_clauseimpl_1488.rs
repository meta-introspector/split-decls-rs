// Generated macro for impl_1488 (impl)
macro_rules! Depcrate_query_builder_where_clauseimpl_1488 {
() => {
// Module: crate::query_builder::where_clause
// Provides: {"impl_1488"}
// Dependencies: {}
impl < 'a , DB , Predicate > WhereOr < Predicate > for BoxedWhereClause < 'a , DB > where DB : Backend + 'a , Predicate : QueryFragment < DB > + Send + 'a , Grouped < Or < Box < dyn QueryFragment < DB > + Send + 'a > , Predicate > > : QueryFragment < DB > , { type Output = Self ; fn or (self , predicate : Predicate) -> Self :: Output { use self :: BoxedWhereClause :: Where ; match self { Where (where_clause) => Where (Box :: new (Grouped (Or :: new (where_clause , predicate)))) , BoxedWhereClause :: None => Where (Box :: new (predicate)) , } } }
};
}
