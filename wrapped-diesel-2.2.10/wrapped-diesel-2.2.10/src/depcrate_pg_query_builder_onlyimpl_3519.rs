// Generated macro for impl_3519 (impl)
macro_rules! Depcrate_pg_query_builder_onlyimpl_3519 {
() => {
// Module: crate::pg::query_builder::only
// Provides: {"impl_3519"}
// Dependencies: {}
impl < S > Table for Only < S > where S : Table + Clone + AsQuery , < S as Table > :: PrimaryKey : SelectableExpression < Only < S > > , < S as Table > :: AllColumns : SelectableExpression < Only < S > > , < S as QuerySource > :: DefaultSelection : ValidGrouping < () > + SelectableExpression < Only < S > > , { type PrimaryKey = < S as Table > :: PrimaryKey ; type AllColumns = < S as Table > :: AllColumns ; fn primary_key (& self) -> Self :: PrimaryKey { self . source . primary_key () } fn all_columns () -> Self :: AllColumns { S :: all_columns () } }
};
}
