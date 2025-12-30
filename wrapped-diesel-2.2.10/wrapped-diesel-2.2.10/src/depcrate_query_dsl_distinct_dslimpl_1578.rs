// Generated macro for impl_1578 (impl)
macro_rules! Depcrate_query_dsl_distinct_dslimpl_1578 {
() => {
// Module: crate::query_dsl::distinct_dsl
// Provides: {"impl_1578"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl < T , Selection > DistinctOnDsl < Selection > for T where Selection : SelectableExpression < T > , T : Table + AsQuery < Query = SelectStatement < FromClause < T > > > , SelectStatement < FromClause < T > > : DistinctOnDsl < Selection > , T :: DefaultSelection : Expression < SqlType = T :: SqlType > + ValidGrouping < () > , T :: SqlType : TypedExpressionType , { type Output = dsl :: DistinctOn < SelectStatement < FromClause < T > > , Selection > ; fn distinct_on (self , selection : Selection) -> dsl :: DistinctOn < Self , Selection > { self . as_query () . distinct_on (selection) } }
};
}
