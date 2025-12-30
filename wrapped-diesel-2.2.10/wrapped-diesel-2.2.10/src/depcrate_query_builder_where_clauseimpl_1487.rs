// Generated macro for impl_1487 (impl)
macro_rules! Depcrate_query_builder_where_clauseimpl_1487 {
() => {
// Module: crate::query_builder::where_clause
// Provides: {"impl_1487"}
// Dependencies: {}
impl < 'a , DB , Predicate > WhereAnd < Predicate > for BoxedWhereClause < 'a , DB > where DB : Backend + 'a , Predicate : QueryFragment < DB > + Send + 'a , Grouped < And < Box < dyn QueryFragment < DB > + Send + 'a > , Predicate > > : QueryFragment < DB > , { type Output = Self ; fn and (self , predicate : Predicate) -> Self :: Output { use self :: BoxedWhereClause :: Where ; match self { Where (where_clause) => Where (Box :: new (Grouped (And :: new (where_clause , predicate)))) , BoxedWhereClause :: None => Where (Box :: new (predicate)) , } } }
};
}
