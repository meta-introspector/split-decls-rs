// Generated macro for impl_3563 (impl)
macro_rules! Depcrate_pg_query_builder_tablesampleimpl_3563 {
() => {
// Module: crate::pg::query_builder::tablesample
// Provides: {"impl_3563"}
// Dependencies: {}
impl < S , TSM > AsQuery for Tablesample < S , TSM > where S : Table + Clone , TSM : TablesampleMethod , < S as QuerySource > :: DefaultSelection : ValidGrouping < () > + SelectableExpression < Tablesample < S , TSM > > , { type SqlType = < < Self as QuerySource > :: DefaultSelection as Expression > :: SqlType ; type Query = SelectStatement < FromClause < Self > > ; fn as_query (self) -> Self :: Query { SelectStatement :: simple (self) } }
};
}
