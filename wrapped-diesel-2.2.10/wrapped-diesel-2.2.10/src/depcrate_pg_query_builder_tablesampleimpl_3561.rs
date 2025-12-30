// Generated macro for impl_3561 (impl)
macro_rules! Depcrate_pg_query_builder_tablesampleimpl_3561 {
() => {
// Module: crate::pg::query_builder::tablesample
// Provides: {"impl_3561"}
// Dependencies: {}
impl < S , TSM > QuerySource for Tablesample < S , TSM > where S : Table + Clone , TSM : TablesampleMethod , < S as QuerySource > :: DefaultSelection : ValidGrouping < () > + SelectableExpression < Tablesample < S , TSM > > , { type FromClause = Self ; type DefaultSelection = < S as QuerySource > :: DefaultSelection ; fn from_clause (& self) -> Self :: FromClause { self . clone () } fn default_selection (& self) -> Self :: DefaultSelection { self . source . default_selection () } }
};
}
