// Generated macro for impl_1797 (impl)
macro_rules! Depcrate_query_source_aliasing_dsl_implsimpl_1797 {
() => {
// Module: crate::query_source::aliasing::dsl_impls
// Provides: {"impl_1797"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl < S , Selection > DistinctOnDsl < Selection > for Alias < S > where S : AliasSource , Selection : SelectableExpression < Self > , Self : QuerySource + AsQuery < Query = SelectStatement < FromClause < Self > > > , SelectStatement < FromClause < Self > > : DistinctOnDsl < Selection > , < Self as QuerySource > :: DefaultSelection : Expression < SqlType = < Self as AsQuery > :: SqlType > + ValidGrouping < () > , < Self as AsQuery > :: SqlType : TypedExpressionType , { type Output = dsl :: DistinctOn < SelectStatement < FromClause < Self > > , Selection > ; fn distinct_on (self , selection : Selection) -> dsl :: DistinctOn < Self , Selection > { DistinctOnDsl :: distinct_on (self . as_query () , selection) } }
};
}
