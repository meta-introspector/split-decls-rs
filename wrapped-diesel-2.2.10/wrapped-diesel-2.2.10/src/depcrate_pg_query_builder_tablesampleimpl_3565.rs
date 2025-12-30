// Generated macro for impl_3565 (impl)
macro_rules! Depcrate_pg_query_builder_tablesampleimpl_3565 {
() => {
// Module: crate::pg::query_builder::tablesample
// Provides: {"impl_3565"}
// Dependencies: {}
impl < S , TSM > Table for Tablesample < S , TSM > where S : Table + Clone + AsQuery , TSM : TablesampleMethod , < S as Table > :: PrimaryKey : SelectableExpression < Tablesample < S , TSM > > , < S as Table > :: AllColumns : SelectableExpression < Tablesample < S , TSM > > , < S as QuerySource > :: DefaultSelection : ValidGrouping < () > + SelectableExpression < Tablesample < S , TSM > > , { type PrimaryKey = < S as Table > :: PrimaryKey ; type AllColumns = < S as Table > :: AllColumns ; fn primary_key (& self) -> Self :: PrimaryKey { self . source . primary_key () } fn all_columns () -> Self :: AllColumns { S :: all_columns () } }
};
}
