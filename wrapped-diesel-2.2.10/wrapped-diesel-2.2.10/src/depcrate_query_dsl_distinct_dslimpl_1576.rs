// Generated macro for impl_1576 (impl)
macro_rules! Depcrate_query_dsl_distinct_dslimpl_1576 {
() => {
// Module: crate::query_dsl::distinct_dsl
// Provides: {"impl_1576"}
// Dependencies: {}
impl < T > DistinctDsl for T where T : Table + AsQuery < Query = SelectStatement < FromClause < T > > > , T :: DefaultSelection : Expression < SqlType = T :: SqlType > + ValidGrouping < () > , T :: SqlType : TypedExpressionType , { type Output = dsl :: Distinct < SelectStatement < FromClause < T > > > ; fn distinct (self) -> dsl :: Distinct < SelectStatement < FromClause < T > > > { self . as_query () . distinct () } }
};
}
